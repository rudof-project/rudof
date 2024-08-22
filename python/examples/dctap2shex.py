from pyrudof import convert

print("Reading file from examples/simple.csv and generating output in target/simple.shex")

convert.dctap2shex("examples/simple.csv", "target/simple.shex")

