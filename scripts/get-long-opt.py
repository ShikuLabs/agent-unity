import os

absolute_path = os.path.realpath(__file__)
absolute_dir = os.path.dirname(absolute_path)

print(__file__)
print(absolute_path)
print(absolute_dir)