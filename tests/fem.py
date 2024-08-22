from automesh import Spn


nel = [3, 5, 4]
scale = [1.2, 2.3, 0.4]
translate = [-0.3, 1.1, 0.5]


def test_write_inp():
    spn = Spn.from_spn('tests/input/f.spn', nel)
    fem = spn.as_finite_elements(scale, translate)
    fem.write_inp("target/f.inp")
    assert False
