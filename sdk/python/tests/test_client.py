"""Tests for the remem Python SDK."""

import pytest


class TestMemoryClient:
    """Unit tests for the Memory client (mocked HTTP)."""

    def test_import(self):
        """Verify the SDK can be imported."""
        from remem import Memory, MemoryResult, StoreResponse

        assert Memory is not None
        assert MemoryResult is not None
        assert StoreResponse is not None

    def test_models(self):
        """Verify Pydantic models work."""
        from remem.models import MemoryType, ForgetMode

        assert MemoryType.FACT == "fact"
        assert MemoryType.PROCEDURE == "procedure"
        assert ForgetMode.DELETE == "delete"
        assert ForgetMode.ARCHIVE == "archive"

    def test_config_defaults(self):
        """Verify config defaults are sensible."""
        from remem.config import RememConfig

        config = RememConfig()
        assert config.base_url == "http://localhost:7474"
        assert config.project == "default"
        assert config.timeout == 30.0
