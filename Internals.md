# How stuff works

## what is a Series?
* A Series simply backed by an Ndarray which stores
Data and a vector of strings acting as the index *this is subject to change once the index trait has been stabilized*

> Most operations are directly applied on the ndarray while some functions are applied to the index eg `drop`

> Series are not inheritingly complex and implementing new function is trivial.

## What's a DataFrame

> It's the definition of complex
* DataFrames are actually amazing. 
A DataFrame is just the icing on a `BlockManager` which maintains communication between
the DataFrame and `Blocks`

  ### What is a blockmanager?
  * Block managers implement a basic abstraction layer that allows
Functions or data to be implemented on other deeper structs called `Blocks`
A block manager defines what function can act on what block and what is expected back from the function
It allows homogeneous functions to work on heterogeneous data *with the caveat that the new block formed from the data will 
contain only data yeilded from the homogeneous function*
  * Block managers also reduce time spent on executing certain functions since all Series\<T> are contained in one Block another Series\<P> will be held in another block
so if a function only acts on type `T` it will just act on Block\<T> 
  * Block managers allow for parrallelized methods on blocks which **greatly improves speed**
 Especially on computationally expensive operations that do not depend on each other (eg Fourier transform on a DataFrame)


  ### What is a block
  * A container for one dimension homogeneous Series ( A block is a Vec\<Series\<f64>> for example)
.They are the bare bones of a DataFrame holding all similar Series type

  * Any function that can benefit from parallel iterators is by default parallelized to gain the performance benefit(Rayon is sweet)

## Implementing new functionality for DataFrame
  * * * Start at the block where actual computations happen
 * * Move to the block manager where higher level operations `like determining what type the block function will work on`
 * Put the greatest documentation on the DataFrame implementation and then call the underlying blockmanager implementation which should handle the rest

**Write understandable documentation**
 Comment code where possible

Love ☕☕
