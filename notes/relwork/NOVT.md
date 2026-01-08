# NoVT Relwork Comparison

[paper](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=9581255)

[github](https://github.com/novt-vtable-less-compiler/novt-llvm/tree/master)

Summary:

NoVT replaces dynamic dispatch function calls (in C++) with static function
calls using CHA. 

Goal: prevent vtable hijacking. Security gain only if vtables are completely 
eliminated.

In particular, this seems like the most-recent (double check this) CHA use-case
and the code is pretty accessible, so using to understand how CHA is implemented
(and also why my naive benchmark performs so poorly). 

## Implementation

Mangles class names for making inheritance explicit?
- https://github.com/novt-vtable-less-compiler/novt-llvm/commit/d6bbce125e5124534c17a2296597ffde13edd11a#diff-c5938d463466edb1ec98fb02863b6a13ab995e047fc84211bc9998611fb4d059

Main logic?
- https://github.com/novt-vtable-less-compiler/novt-llvm/blob/master/llvm-novt/lib/Transforms/IPO/NoVTAbiBuilder.cpp

Calculate which classes to include in switch statement
- https://github.com/novt-vtable-less-compiler/novt-llvm/blob/master/llvm-novt/lib/Transforms/IPO/NoVTAbiBuilder.cpp#L456
- get a vec of `DefClass`es

```c++
    std::vector<DefClass *> getInheritancePath(DefClassIdent *ident) {
      std::vector<DefClass *> result;
      while (ident) {
        result.push_back(ident->cls);
        if (ident->bases.empty()) break;
        ident = ident->firstBase;
      }
      return result;
```

`SwitchFunction` Class
- https://github.com/novt-vtable-less-compiler/novt-llvm/blob/master/llvm-novt/lib/Transforms/IPO/NoVTAbiBuilder.cpp#L501
- where is this class initialized? i.e., what are examples of its fields? what
  is a switch function?? is this how one performs a switch? or is it like a map
  between a dynamically-dispatched function (like, `foo()`) and a switch-case
  statement setup? or something else?

```c++
    class SwitchFunction {
    public:
        const SwitchFunctionType type;
        Function *const function;
        SwitchInst *const switchInst;
        const bool defaultIsImportant;
        bool removed = false;

        std::map<DefClassIdent *, BasicBlock *> cases;
        std::set<BasicBlock *> blocks;
        std::set<DefClassIdent *> idents;
```

populating class hierarchy info
- https://github.com/novt-vtable-less-compiler/novt-llvm/blob/master/llvm-novt/lib/Transforms/IPO/NoVTAbiBuilder.cpp#L606

`FunctionBuilder`
- related to `SwitchFunction`...
- https://github.com/novt-vtable-less-compiler/novt-llvm/blob/master/llvm-novt/lib/Transforms/IPO/NoVTAbiBuilder.cpp#L1013

```c++

        template<class CacheType>
        struct FunctionBuilder {
        private:
            std::map<CacheType, BasicBlock *> cache;
        public:
            const SwitchFunctionType type;
            Module &M;
            LLVMContext &C;
            Function *function;
            int thisArgNumber = 0;
            SwitchInst *switchInst = nullptr;
            std::set<DefClassIdent *> visitedIdents;
            SwitchFunction *swfunc = nullptr;
            bool defaultIsImportant = false;
```

what is the `Function` type?




