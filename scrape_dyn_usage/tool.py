import argparse
import os
import re

# FIXME not storing dyn <trait> + <...> (additional trait bounds)

class State:

    def __init__(self, rootdir):
        self.rootdir = rootdir
        self.dyn_matches = 0
        self.impl_matches = 0
        self.dyn_traits = set()
        self.dyn_trait_impls = {}

    def search_file_for_dyn(self, filename):
        with open(filename) as file:
            for line_num, line in enumerate(file, start=1):
                # TODO once see "mod tests" skip rest of file
                line = line.rstrip()
                line = line.lstrip()
                comment_match = re.search("^[/]{2,3}", line)
                pound_match = re.search("^[#]", line)
                if not comment_match and not pound_match: 
                    dyn_match = re.search("dyn ([A-Za-z:_]+)", line)
                    if dyn_match:
                        self.dyn_matches += 1
                        print("~~MATCH (", self.dyn_matches, ") : ", dyn_match.group(1))
                        print("file: ", filename, ":", line_num)
                        print("line: ", line)
                        print()
                        traitname = dyn_match.group(1)
                        self.dyn_traits.add(traitname)
    
    def search_dyn(self):
        for root, subdirs, files in os.walk(self.rootdir):
            for filename in files:
                if not filename.endswith(".rs"):
                    continue
                absfilename = root + "/" + filename
                if "tests/" in absfilename or "benches/" in absfilename or "examples/" in absfilename:
                    continue
                self.search_file_for_dyn(absfilename)
    
    def search_file_for_impl(self, filename):
        with open(filename) as file:
            for line_num, line in enumerate(file, start=1):
                # TODO once see "mod tests" skip rest of file
                line = line.rstrip()
                line = line.lstrip()
                comment_match = re.search("^[/]{2,3}", line)
                pound_match = re.search("^[#]", line)
                if not comment_match and not pound_match: 
                    impl_match = re.search("impl([<][A-Za-z,'\s]+[>])? ([A-Za-z]+)([<][A-Za-z,'\s]+[>])? for ([A-Za-z:_]+([<][A-Za-z,'\s]+[>])?)", line)
                    if impl_match and impl_match.group(2) in self.dyn_traits:
                        dyn_trait = impl_match.group(2)
                        dyn_trait_impl = impl_match.group(4)
                        self.impl_matches += 1
                        print("~~MATCH (", self.impl_matches, ") : ", dyn_trait_impl)
                        print("TRAIT: ", dyn_trait)
                        print("file: ", filename, ":", line_num)
                        print("line: ", line)
                        print()
                        if dyn_trait in self.dyn_trait_impls:
                            updated = self.dyn_trait_impls[dyn_trait]
                            updated.add(dyn_trait_impl)
                            self.dyn_trait_impls[dyn_trait] = updated
                        else:
                            self.dyn_trait_impls[dyn_trait] = {dyn_trait_impl}
    
    def search_impl(self): 
        for root, subdirs, files in os.walk(self.rootdir):
            for filename in files:
                if not filename.endswith(".rs"):
                    continue
                absfilename = root + "/" + filename
                if "tests/" in absfilename or "benches/" in absfilename or "examples/" in absfilename:
                    continue
                self.search_file_for_impl(absfilename)

    def print(self):
        print("Traits: ", s.dyn_traits)
        print("Impls: ", s.dyn_trait_impls)
        for impl in s.dyn_trait_impls:
            print(impl, ": \t", len(s.dyn_trait_impls[impl]))

def arg_parse():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dir", "-d",
        type=str,
        required=True,
        help="directory in which to search for dyn usage")
    args = parser.parse_args()
    return args.dir

if __name__ == "__main__":
    path = arg_parse()
    abspath = os.path.abspath(path)
    s = State(abspath)

    if os.path.isdir(abspath):
        print("\n-----DYN SEARCH\n")
        s.search_dyn()
        print("\n-----IMPL SEARCH\n")
        s.search_impl()
    else: 
        print("\n-----DYN SEARCH\n")
        s.search_file_for_dyn()
        print("\n-----IMPL SEARCH\n")
        s.search_file_for_impl()

    print("\n-----TOTAL\n")
    print("num dyn matches: ", s.dyn_matches, "\n")
    print("num impl matches: ", s.impl_matches, "\n")
    if bool(s.dyn_traits) and bool(s.dyn_trait_impls):
        group4 = False
        for impl in s.dyn_trait_impls:
            if len(s.dyn_trait_impls[impl]) != 1:
                group4 = True
        if group4:
            print("group 4")
            s.print()
        else:
            print("group 3")
            s.print()
    elif not bool(s.dyn_traits) and not bool(s.dyn_trait_impls):
        print("group 1")
        s.print()
    else:
        print("group 2")
        s.print()


