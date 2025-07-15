import sys
import os
import re

dyn_traits = set()
dyn_trait_impls = {}

# FIXME not storing dyn <trait> + <...> (additional trait bounds)

def search_file_for_dyn(filename):
    with open(filename) as file:
        for line in file: 
            line = line.rstrip()
            comment_match = re.search("^\s*[/]{2,3}", line)
            if not comment_match: 
                dyn_match = re.search("dyn ([A-Za-z:]+)", line)
                if dyn_match:
                    #print("line: ", line)
                    #print("DYN MATCH: ", dyn_match.group(1))
                    traitname = dyn_match.group(1)
                    dyn_traits.add(traitname)

def search_dyn(rootdir):
    for root, subdirs, files in os.walk(rootdir):
        for filename in files:
            absfilename = root + "/" + filename
            search_file_for_dyn(absfilename)

def search_file_for_impl(filename):
    with open(filename) as file:
        for line in file: 
            line = line.rstrip()
            comment_match = re.search("^\s*[/]{2,3}", line)
            if not comment_match: 
                impl_match = re.search("impl([<][A-Za-z,'\s]+[>])? ([A-Za-z]+)([<][A-Za-z,'\s]+[>])? for ([A-Za-z])", line)
                if impl_match and impl_match.group(2) in dyn_traits:
                    dyn_trait = impl_match.group(2)
                    dyn_trait_impl = impl_match.group(4)
                    #print("IMPL MATCH: ", dyn_trait, dyn_trait_impl)
                    if dyn_trait in dyn_trait_impls:
                        updated = dyn_trait_impls[dyn_trait]
                        updated.append(dyn_trait_impl)
                        dyn_trait_impls[dyn_trait] = updated
                    else:
                        dyn_trait_impls[dyn_trait] = [dyn_trait_impl]

def search_impl(rootdir): 
    for root, subdirs, files in os.walk(rootdir):
        for filename in files:
            absfilename = root + "/" + filename
            search_file_for_impl(absfilename)

if __name__ == "__main__":
    path = sys.argv[1]
    abspath = os.path.abspath(path)

    if os.path.isdir(abspath):
        search_dyn(abspath)
        search_impl(abspath)
    else: 
        search_file_for_dyn(abspath)
        search_file_for_impl(abspath)

    print(dyn_traits)

    print(dyn_trait_impls)






