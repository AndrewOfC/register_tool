


from register_config_validator import Validator


def test_goodregisters():
    v = Validator()
    (errors, warnings, count) = v.validate("data/testconfig.yaml")
    assert len(errors) == 0
    assert len(warnings) == 0
    assert count > 0

def test_missing_parent():
    v = Validator()
    (errors, warnings, count) = v.validate("data/testconfig.yaml", root_key="missing_parent")
    assert len(errors) > 0
    assert errors[0].index("not found") != -1

def test_other():
    v = Validator(verbose=True)
    (errors, warnings, count) = v.validate("data/testconfig.yaml", root_key="other")
    assert len(errors) > 0
    assert errors[0].index("not found") != -1

