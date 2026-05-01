import asyncio
import os
from datetime import datetime
from uuid import uuid4
from remem import Memory
from remem.models import MemoryType

async def run_simulation():
    print("🚀 Starting Remem End-to-End Agent Simulation...")
    
    # Initialize client
    # We'll use the 'mock' provider so we don't need real keys
    async with Memory(project="simulation-alice", base_url="http://localhost:7474") as m:
        session_id = f"session-{uuid4().hex[:8]}"
        print(f"📂 Created session: {session_id}")
        
        # --- Day 1: Alice introduces herself ---
        print("\n--- Day 1: User Introduction ---")
        facts = [
            "My name is Alice.",
            "I am a software engineer focused on Rust and systems programming.",
            "I prefer dark mode in all my applications.",
            "I live in Berlin, but I'm originally from Vancouver."
        ]
        
        for fact in facts:
            print(f"💾 Storing: {fact}")
            await m.store(fact, tags=["bio", "preferences"])
            
        # --- Day 2: Recalling and using context ---
        print("\n--- Day 2: Context Recall ---")
        query = "What do we know about Alice's programming background?"
        print(f"🔍 Recalling: {query}")
        results = await m.recall(query)
        
        for i, res in enumerate(results):
            print(f"  {i+1}. [{res.importance:.1f}] {res.content}")
            if res.reasoning:
                print(f"     💡 Reason: {res.reasoning}")
        
        # --- Day 3: A contradiction appears ---
        print("\n--- Day 3: Contradiction Check ---")
        print("💾 Storing conflicting info: 'Alice actually works in Python now.'")
        # In a real scenario, the consolidation pass would detect this
        await m.store("Alice actually works in Python now.", tags=["work"])
        
        # --- Day 4: Consolidation ---
        print("\n--- Day 4: Consolidation Pass ---")
        print(f"⚙️ Triggering consolidation for {session_id}...")
        report = await m.consolidate(session_id)
        
        print(f"📊 Consolidation Report:")
        print(f"   - New facts: {report.new_facts}")
        print(f"   - Updated facts: {report.updated_facts}")
        print(f"   - Contradictions detected: {len(report.contradictions)}")
        
        for c in report.contradictions:
            print(f"   ⚠️ Contradiction: {c.explanation}")
            
        # --- Day 5: Final State ---
        print("\n--- Day 5: Final Knowledge State ---")
        final_query = "What is Alice's current focus?"
        final_results = await m.recall(final_query)
        for i, res in enumerate(final_results):
            print(f"  {i+1}. {res.content}")

if __name__ == "__main__":
    try:
        asyncio.run(run_simulation())
    except Exception as e:
        print(f"❌ Simulation failed: {e}")
        print("\nTip: Make sure the remem server is running with 'cargo run -p remem-api -- --project simulation-alice'")
