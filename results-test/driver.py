import sys
import argparse
import os
import subprocess

group1 = set()
group2 = set()
group3 = set()
group4 = set()

def arg_parse():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dir", "-d",
        type=str,
        required=True,
        help="root directory in which to search for dyn usage")
    args = parser.parse_args()
    return args.dir

if __name__ == "__main__":
    rootdir = arg_parse()
    absrootdir = os.path.abspath(rootdir)

    for root, subdirs, files in os.walk(absrootdir):
        if "src" not in subdirs:
            continue
        output_bytes = subprocess.check_output(["python", "tool.py", "-d", root])
        output = output_bytes.decode("utf-8")

        cratename = root.split('/')[-1]

        if "1" in output:
            group1.add(cratename)
        elif "2" in output: 
            group2.add(cratename)
        elif "3" in output: 
            group3.add(cratename)
        elif "4" in output:
            group4.add(cratename)
        else: 
            print("none? ", output)

    print("\nGroup 1 (", len(group1), "): ", group1)
    print("\nGroup 2 (", len(group2), "): ", group2)
    print("\nGroup 3 (", len(group3), "): ", group3)
    print("\nGroup 4 (", len(group4), "): ", group4)

