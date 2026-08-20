# Fixed category -> color mapping, reused across all future plots in this
# investigation so the same category always gets the same color regardless
# of which other categories appear alongside it.
CATEGORY_COLORS = {
    "BbStatements": "black",
    "BbTerminator": "black",
    "BbTotal": "black",

    "StmtContainsDyn":              "tab:blue",
    "StmtCurConstraints":           "tab:orange",
    "StmtLiftTraitobj":             "tab:green",
    "StmtNewConstraints":           "tab:red",
    "StmtNewConstraintsRef":        "tab:purple",
    "StmtNewConstraintsStatic":     "tab:brown",
    "StmtNewConstraintsFromConvert":"tab:blue",
    "StmtSetScoped":                "tab:gray",
    "StmtWriteFields":              "tab:olive",

    "TermDirectCall":         "tab:cyan",
    "TermIndirectCall":       "gold",
    "TermReturn":             "tab:red",
    "TermSwitch":             "crimson",
    "TermReturnScopedGet":    "slateblue",
    "TermReturnFinishFrame":  "sienna",
    "TermFinishFrameReinterp":"deeppink",
    "TermFinishFrameRevisit": "tab:orange",

    "TermCollectResolvedArgs": "steelblue",
    "TermResolveArgs":         "tab:purple",
    "TermInterpStaticCall":    "tab:green",
    "TermInterpVirtualCall":   "firebrick",
    "TermBuildParamSummary":   "mediumorchid",
    "TermParamSummary":        "mediumorchid",
    "TermMemo":                "teal",

    "TermGetImplsCha":              "darkkhaki",
    "TermGetImplsFsa":              "cadetblue",
    "TermVirtualCallPrep":          "indigo",
    "TermSimulateCallPrep":         "gold",
    "TermSimulateRecursiveFallback":"lightseagreen",
    "TermSimulateStdlibStub":       "tab:olive",
    "TermSimulateRealCall":         "tab:gray",
    "TermSimulateMergeResults":     "tab:pink",
    "TermSimulateLoopMergeResults": "navy",

    "TermMergeCloneCstores":        "#9a6324",  # brown
    "TermMergeCstoresMerge":        "#46f0f0",  # cyan
    "TermMergeIdentityCheck":       "#800000",  # maroon
    "TermMergePerKeyMapvals":       "#ffe119",  # yellow/gold
    "TermMergeConstraintsAppend":   "#f032e6",  # magenta
    "TermMergeConstraintsWiden":    "#000075",  # navy
    "TermMergeRefsUnion":           "#bcf60c",  # lime
    "TermMergeWtosUnion":           "#008080",  # teal
    "TermMergeContextsSetup":       "#e6194b",  # red (bright)
    "TermMergeNewContext":          "#fabebe",  # light pink
    "Take":                         "chocolate",
    "VecConstruction":              "#4363d8",  # strong blue
    "TermMergeWtosClone":           "crimson",
    "TermMergeRefsClone":           "darkviolet",
    "TermVirtualMemo":              "springgreen",
    "TermMergeMapsvalsMerge":       "deepskyblue",
    "TermMergeWtosCloneInner":      "darkorange",
}

def color_for(cat):
    return CATEGORY_COLORS.get(cat, "black")
