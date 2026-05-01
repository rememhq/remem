import { test, describe, mock, afterEach } from "node:test";
import assert from "node:assert";
import { Memory } from "./index.js";

describe("Memory SDK", () => {
  afterEach(() => {
    mock.restoreAll();
  });

  test("store() sends correct payload", async () => {
    const memory = new Memory({ project: "test", baseUrl: "http://localhost:7474" });
    
    mock.method(global, "fetch", async () => {
      return {
        ok: true,
        json: async () => ({ id: "123", importance: 5.0, tags: ["test"], created_at: new Date().toISOString() })
      };
    });

    const response = await memory.store("Hello World", { tags: ["test"] });
    assert.strictEqual(response.id, "123");
    assert.strictEqual(response.importance, 5.0);
    
    const fetchCall = (global.fetch as any).mock.calls[0];
    const url = fetchCall.arguments[0];
    const init = fetchCall.arguments[1];
    
    assert.strictEqual(url, "http://localhost:7474/v1/memories");
    assert.strictEqual(init.method, "POST");
    assert.strictEqual(JSON.parse(init.body).content, "Hello World");
    assert.deepStrictEqual(JSON.parse(init.body).tags, ["test"]);
  });

  test("recall() sets query params correctly", async () => {
    const memory = new Memory({ project: "test", baseUrl: "http://localhost:7474" });
    
    mock.method(global, "fetch", async () => {
      return {
        ok: true,
        json: async () => ([{ id: "123", content: "Hello", importance: 5.0, tags: [], memory_type: "fact", created_at: new Date().toISOString(), similarity: 0.9 }])
      };
    });

    const response = await memory.recall("test query", { limit: 5, filter_tags: ["t1", "t2"] });
    assert.strictEqual(response.length, 1);
    
    const fetchCall = (global.fetch as any).mock.calls[0];
    const url = fetchCall.arguments[0];
    
    assert.ok(url.includes("q=test+query"));
    assert.ok(url.includes("limit=5"));
    assert.ok(url.includes("filter_tags=t1%2Ct2"));
  });

  test("consolidate() sends correct session and model", async () => {
    const memory = new Memory({ project: "test", baseUrl: "http://localhost:7474" });
    
    mock.method(global, "fetch", async () => {
      return {
        ok: true,
        json: async () => ({ session_id: "s1", new_facts: 2, updated_facts: 0, contradictions: [], knowledge_graph_updates: [] })
      };
    });

    const response = await memory.consolidate("s1", "gemini-1.5-flash");
    assert.strictEqual(response.new_facts, 2);
    
    const fetchCall = (global.fetch as any).mock.calls[0];
    const url = fetchCall.arguments[0];
    const init = fetchCall.arguments[1];
    
    assert.strictEqual(url, "http://localhost:7474/v1/sessions/s1/consolidate");
    assert.strictEqual(init.method, "POST");
    assert.strictEqual(JSON.parse(init.body).model, "gemini-1.5-flash");
  });
});
