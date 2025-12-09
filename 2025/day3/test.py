import pytest

from .main import max_for_bank

testdata = [
    (987654321111111, 98),
    (811111111111119, 89),
    (234234234234278, 78),
    (818181911112111, 92),
]


@pytest.mark.parametrize("bank_as_num,expected", testdata)
def test_bank_two_batteries(bank_as_num, expected):
    bank = list(map(int, str(bank_as_num)))
    assert max_for_bank(bank) == expected
