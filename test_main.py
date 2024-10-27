# """
"""
Test goes here
"""
from main import main_res


def test_extract():
    assert main_res()["extract_to"] == "HR_1.csv", "Test for extract() failed"


def test_transform():
    assert main_res()["transform_db"] == "HR_1.db", "Test for load() failed"


def test_create():
    assert main_res()["create"] == "Create Success", "Test for queryCreate() failed"


def test_read():
    assert main_res()["read"] == "Read Success", "Test for queryRead() failed"


def test_update():
    assert main_res()["update"] == "Update Success", "Test for queryUpdate() failed"


def test_delete():
    assert main_res()["delete"] == "Delete Success", "Test for queryDelete() failed"


if __name__ == "__main__":
    test_extract()
    test_transform()
    test_create()
    test_read()
    test_update()
    test_delete()
    print("All tests passed!")


# Test goes here
# """
# from main import main_res


# def test_func():
#     return main_res()


# if __name__ == "__main__":
#     # Test each expected output
#     assert test_func()["extract_to"] == "HR.csv", "Test for extract() failed"
#     assert test_func()["transform_db"] == "HR.db", "Test for load() failed"
#     assert test_func()["create"] == "Create Success", "Test for queryCreate() failed"
#     assert test_func()["read"] == "Read Success", "Test for queryRead() failed"
#     assert test_func()["update"] == "Update Success", "Test for queryUpdate() failed"
#     assert test_func()["delete"] == "Delete Success", "Test for queryDelete() failed"

#     print("All tests passed!")