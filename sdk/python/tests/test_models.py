"""Unit tests for remem SDK Pydantic models."""
from remem.models import MemoryType, MemoryRecord, RecallResult


class TestMemoryType:
    def test_fact_value(self):
        assert MemoryType.FACT == "fact"

    def test_procedure_value(self):
        assert MemoryType.PROCEDURE == "procedure"

    def test_preference_value(self):
        assert MemoryType.PREFERENCE == "preference"

    def test_decision_value(self):
        assert MemoryType.DECISION == "decision"


class TestMemoryRecord:
    def test_create_minimal(self):
        record = MemoryRecord(
            id="00000000-0000-0000-0000-000000000001",
            content="test content",
            importance=5.0,
            tags=[],
            memory_type=MemoryType.FACT,
            created_at="2026-01-01T00:00:00Z",
        )
        assert record.content == "test content"
        assert record.importance == 5.0

    def test_create_with_tags(self):
        record = MemoryRecord(
            id="00000000-0000-0000-0000-000000000001",
            content="tagged",
            importance=5.0,
            tags=["a", "b"],
            memory_type=MemoryType.FACT,
            created_at="2026-01-01T00:00:00Z",
        )
        assert len(record.tags) == 2
        assert "a" in record.tags

    def test_serialization(self):
        record = MemoryRecord(
            id="00000000-0000-0000-0000-000000000001",
            content="test",
            importance=5.0,
            tags=[],
            memory_type=MemoryType.FACT,
            created_at="2026-01-01T00:00:00Z",
        )
        data = record.model_dump()
        assert "content" in data
        assert "importance" in data


class TestRecallResult:
    def test_create(self):
        result = RecallResult(
            id="00000000-0000-0000-0000-000000000001",
            content="recalled content",
            importance=7.0,
            tags=["test"],
            memory_type=MemoryType.FACT,
            created_at="2026-01-01T00:00:00Z",
            similarity=0.92,
            reasoning="Relevant match",
        )
        assert result.similarity == 0.92
        assert result.reasoning == "Relevant match"

    def test_optional_reasoning(self):
        result = RecallResult(
            id="00000000-0000-0000-0000-000000000001",
            content="no reasoning",
            importance=5.0,
            tags=[],
            memory_type=MemoryType.FACT,
            created_at="2026-01-01T00:00:00Z",
            similarity=0.5,
        )
        assert result.reasoning is None
