# libsbrain
A library for execution of Semantic Brain, based on Urban Müller's famous but unprintable language.

### What is SBrain?
  SBrain, or Semantic Brain, is a language based on Urban Müller's famous language with only 8 symbols (3 bit instructions). SBrain's additions increase the number of symbols to 16 (5 bit instructions) and adds a register.

### Specification
  SBrain requires:
  
* a tape datastructure which is addressable up to, at minimum, 65,535 (0x0 - 0xFFFF) cells. Not all of these must be active in memory; however, SBrain programs may assume that they are addressable. They must be initially set to zero unless set with an initialization instruction.
* a stack (FILO) datastructure which must support, at minimum, 256 values. Not all of these must be active in memory; however, SBrain programs may assume that they are addressable. They must be initially set to zero.
