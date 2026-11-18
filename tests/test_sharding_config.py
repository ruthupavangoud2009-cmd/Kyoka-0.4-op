import pytest

from src.core.config import _parse_shard_count, _parse_shard_ids


def test_shard_count_defaults_to_auto_discovery():
    assert _parse_shard_count(None) is None
    assert _parse_shard_count("") is None


def test_shard_count_requires_positive_integer():
    assert _parse_shard_count("8") == 8
    with pytest.raises(ValueError):
        _parse_shard_count("0")
    with pytest.raises(ValueError):
        _parse_shard_count("two")


def test_shard_ids_parse_and_reject_duplicates():
    assert _parse_shard_ids("0, 2,3") == (0, 2, 3)
    assert _parse_shard_ids("") == ()
    with pytest.raises(ValueError):
        _parse_shard_ids("1,1")


def test_shard_ids_reject_non_integer_values():
    with pytest.raises(ValueError):
        _parse_shard_ids("1,x")
