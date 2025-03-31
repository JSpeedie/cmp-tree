# cmp-tree

### Project Description

This repo contains multiple different implementations of the same program:
`cmp-tree`. The program functions as a sort of combination between `cmp` and
`tree`, allowing you to compare the full contents of two directories and find
any disparities.

<details><summary><i>Example (Click to expand)</i></summary>  
Let's say we're in a directory that itself contains two example directories:

```bash
ls -1
```
```
first-dir
second-dir
```

If we look at the directory tree of the first directory we see something like
this:

```bash
tree first-dir
```
```
first-dir
├── G2ME-Usage.md
├── Home.md
├── images
│   ├── G2ME-GUI.png
│   └── G2ME-terminal-demo.gif
└── Walkthroughs.md
```

If we look at the directory tree of the second directory we see something like
this:

```bash
tree second-dir
```
```
second-dir
├── G2ME-Usage.md
├── Home.md
├── images
│   └── G2ME-GUI.png
└── Walkthroughs.md
```

We can see that the second directory tree is missing the
`images/G2ME-terminal-demo.gif` file. Let's also assume that while both
directory trees contain `images/G2ME-GUI.png` that the one in `first-dir` is
not byte-for-byte identical with the one in `second-dir`. In this example,
`cmp-tree` will inform the caller about how `images/G2ME-terminal-demo.gif`
exists in the directory tree rooted at `first-dir`, but not in the directory
tree rooted at `second-dir` and about how `images/G2ME-GUI.png` is not
byte-for-byte identical between the two directory trees.
</details>

&nbsp;

### Installation and Running

#### Bash

```bash
cd bash/cmp-tree
./cmp-tree [path-to-first-directory] [path-to-second-directory]
```

#### C

```bash
cd c/cmp-tree
make
./cmp-tree [path-to-first-directory] [path-to-second-directory]
```

#### C++

```bash
cd cpp/cmp-tree
make
./cmp-tree [path-to-first-directory] [path-to-second-directory]
```

#### Rust

```bash
cd rust/cmp-tree
cargo build
target/debug/cmp-tree [path-to-first-directory] [path-to-second-directory]
```

&nbsp;

### Motivation

The impetus for this project came from my need to compare one backup of mine
that had suffered from a filesystem error which I attempted to recover from to
another backup that had not suffered the error. I wanted to make sure that
every single file contained on the recovered backup was byte-for-byte identical
with the corresponding file on the healthy backup.

&nbsp;

### Why are There Multiple Implementations?

The long story short is that `cmp-tree` has served as a project for
self-directed exploration and learning. What started out as the need for a
simple utility expanded into an opportunity to learn something about:

1. The performance differences between Bash and C/C++/Rust
2. How certain objects (like `std::string` and `std::vector`) are implemented
   in C++
3. What it's like developing in C compared to more modern systems-level
   languages like C++/Rust
4. What it's like writing in a very modern language like Rust compared to C++
5. How to profile a program.

If you came here looking only for an executable that will allow you to compare
all the files in two directory trees, then simply build the Rust version of
`cmp-tree` and use that. If you want to hear a little more about what I learned
from this project, then please feel free to read the
[Project Report](#project-report) below.

&nbsp;

### Project Report

Originally I thought this project could be a simple bash script. Run `find` on
the first directory, then `find` on the second directory, combine the results
into one giant list, `sort` it, filter it so it only contains unique paths
(using `uniq`), and then run `cmp` on `[first_directory]/[unique_find_path[i]]`
and `[second_directory]/[unique_find_path[i]]`.

It didn't take me long before I had a working Bash implementation, but it did
seem sort of slow. I looked online and found that `diff -qr [first_directory]
[second_directory]` already accomplished the behaviour I was trying to code in
`cmp-tree`. I compared the speeds between my Bash implementation and `diff
-qr`, and `diff -qr` was significantly faster (by about 3 times). I could have
simply stopped there and used `diff -qr` for my purpose, but I wondered if I
could make a program that could accomplish the task faster than `diff -qr`.
After all, `diff` is a tool for finding differences between two files and
telling you *how* they differ. `cmp-tree`, on the other hand, only needs to
find *if* the files differ.

My next thought was that I should be able to get dramatic time savings by
simply implementing `cmp-tree` in a more performant language like C++. It took
me about twice as long to implement the C++ version, but it was significantly
more enjoyable to write it and there was room for big improvements (such as
multithreading) that might not have been possible or easy in Bash. When I ran
some speed tests, however, I found that my C++ implementation was essentially
just as slow as the Bash implementation! I was shocked. My first thought was
that perhaps this absence of improved performance might come from some of the
"luxury" classes I was using in my C++ implementation (such as
`std::filesystem::path` and `std::vector`). I'd heard in passing that
`std::vector` was not super fast and `std::filesystem::path` seemed to
implement a lot of behaviour I perhaps didn't need for my program. To see if
this was really the case, I set out to implement `cmp-tree` in C.

The C implementation took me easily 4-6 times as long as it took me to write
the C++ implementation. This was mostly because I wrote my own implementations
of: (1) a C++ vector I called `DynamicArray`, (2) a C++ string I called
`String`, (3) basic functions that would allow me to work with my `String`s as
if they were paths, and (4) a hand-written implementation of merge sort that
worked on `DynamicArray`. This was a great learning opportunity in many ways. I
got to experience first hand some of the frustrations with C that might've
motivated C++ such as:

1. Having to hand-write implementations for things most programmers feel
   (perhaps rightfully so) entitled to such as vectors/dynamic arrays, and
   decent strings.
2. Proper support for generics so you can write one implementation of your
   dynamic array instead of one for each type of data your dynamic array needs
   to serve as a container for.
3. Proper support for generics so you can specify the type your dynamic array
   serves as a container for so that your compiler can warn you if you passed a
   dynamic array of ints to a function that only takes a dynamic array of
   strings, for instance.

I learned a lot about how strings and vectors might be implemented in C++, but
I also realized how much C can slow down development simply due to its lack of
basic utilities such as decent strings and vectors. I also saw the benefits of
C such as the fact that while you might have to implement something from
scratch, that implementation is likely bespoke and very well-suited to the
specific use case. Eventually though (and after lots of debugging), I finally
completed my C implementation. When I ran the speed tests I once again found
that it was on par with the Bash implementation in terms of speed!

I quickly realized that this lack of performance from the C implementation must
mean that there was something fundamentally off about the design of my
implementation that must be slowing things down. At this point, both my C++ and
C implementation would use `opendir()` and `readdir()` to generate a list of
relative paths representing every file within a directory, including all the
files in subdirectories. The implementations would then do this again with the
second directory, combine the two file path lists, sort them, filter them to
contain only unique entries and then `fork()` -> `exec()` a `cmp` command on
`[first_directory]/[unique_file_path[i]]` and
`[second_directory]/[unique_file_path[i]]`. Remembering this, I thought that it
was possible that calling `fork()` so many times could be a big part of
problem. After all, it's not free to just duplicate a process. Before making
any changes though, I decided to use `perf` to profile the C implementation.  I
found that the C implementation was getting a lot of L2 and L3 cache misses,
and this seemed like an obvious bottleneck. I remembered hearing recently that
sometimes caches are specific to processes, and if this was indeed the case
then there would be no caching benefit if one process cached memory that
another process also wanted to read. In short, forking so many times during
execution could be slowing down the program in two ways: (1) because forking
itself requires processing time and (2) the many forked processes may not
benefit from the caching of their siblings processes, preventing the program
from benefitting (as much as this program can) from caching.

My solution was to eliminate all forking by implementing a quick-and-dirty
function that would replace a `fork()` -> `exec("cmp
[first_directory]/[unique_file_path[i]]
[second_directory]/[unique_file_path[i]]")` call pair by simply comparing the
contents of two files, and if at any point it found a difference, exiting and
returning a status that indicates that the two files are different. My
first-draft of this function yielded no significant performance gain which
surprised me given that there should be substantially less overhead - no
forking, no commandline argument or flag parsing, none of the extra
functionality a more complete program like `cmp` might have. My first-draft
file-comparison function worked by reading a set number of bytes from both
files, and then looping through those bytes 1 by 1 and if the byte from one
file mismatched the corresponding byte from the other file, it exited early.
After some quick research, I found that using `memcmp()` to compare two sets of
bytes was possibly faster. I gave it a go and found my program was
*substantially* faster. It turns out that on a lot of architectures, `memcmp()`
has an implementation written efficiently in assembly. For example, it could be
vectorized for your processor architecture, giving it huge speed gains over
whatever the compiler was able to do to my C/C++ code for comparing bytes. At
the time of writing, this switch from manually checking each byte to using
`memcmp()` represents the biggest speed improvement I've found so far.

Before making the switch from manually comparing bytes to using `memcmp()`, I
also tried multithreading the C implementation to see if that would help. It
led to modest speed gains, but it also came at the cost of using significantly
more cores and with cache-miss downsides of its own. It seemed to me like
multithreading the program was a bit of a deal with the devil: your program
takes less real-world time to finish (on a typical, multicore personal
computer), but it takes more CPU time per file comparison. In other words,
running on a very busy server or workstation, it seemed to me like the
multithreaded version could actually be slower (in real-world time) due to: (1)
the flat cost of dividing the work and passing it onto threads, (2) the fact
that on a busy CPU, it may be unlikely that any two threads are both working at
the same time, and (3) that there may also be more cache misses because
multiple threads are making non-adjacent, perhaps even distant data accesses.
Worse still, the multithreaded, non-`memcmp()` C implementation (running on my
idle laptop) was **still** multiple factors slower than `diff -qr`! Using
`memcmp()` represented a solution that simply made better use of the underlying
hardware. If the implementation of `memcmp()` was vectorized, the code would
make use of hardware capability that was just underutilized in other
implementations. Unlike with the multithreaded version of `cmp-tree`, with
`memcmp()` there were no considerations to take into account - on a busy CPU or
an idle CPU, using `memcmp()` represented significant and straight forward
performance gains. To top it all off, if it made sense to do so, the benefits
of using `memcmp()` could also be multiplied by multithreading the program.

At this point, I had C and C++ implementations that were around as fast if not
faster than `diff -qr`, so what's with the Rust version? I had 3 motiations for
making a Rust implementation of `cmp-tree`. First, at the time I was very new
to Rust and I thought this could be a good project for helping me familiarize
myself with what it was like to write code in Rust. For example, this project
represented my first chance to work with files and paths in Rust, as well as
serving as my first meaningful attempt to work with Rust's basic data
structures like `String`s and `Vec`s. The second motivation was that this
modestly-sized project would allow me to get a sense of what the working
environment for Rust would be like. What does compilation look like? How is it
creating documentation? How is it adding tests to your code? Of course I could
learn about this on a project of almost any size, but this simple command line
utility felt like a project that would require an appropriate level of
commitment. My final motivation for making a Rust implementation of `cmp-tree`
was to see how Rust compared to C++ and C in terms of performance. Would all
the memory safety come at the cost of slightly worse performance? Would the
promise of many zero-cost abstractions hold true? I wanted to see for myself.

Having implemented the same core functionality of the idea behind `cmp-tree`, I
can now share my thoughts about Rust with respect to those 3 initial
motivations. When it came to writing the Rust implementation of `cmp-tree`, one
major (and surprising) takeaway I had was how quickly I was able to get a
working implementation done in Rust, given my lack of familiarity. Even though
it was my first real project working with files, paths, `String`s and `Vec`s, I
found all of the aforementioned to be very comfortable to work with. I can
honestly say that I preferred writing the code in Rust to writing it in C++,
and that I can definitely say that I preferred writing the code in Rust to
writing it in C! I also really liked Rust's working environment. Compilation is
a breeze because I don't have to write a Makefile or fiddle with CMake.
Compilation is completely handled by `cargo` which figures out what it needs to
do from the Cargo.toml, and the pub, use, etc. keywords. Lastly, when it came
to performance testing, the Rust version was on par with C and C++ (even though
at the time the C version was multithreaded while the Rust version was not!).
In summary, I had a great time writing the code in Rust. To me, it seemed like
there were lots of smart decisions that had been made with the data structures
I was using and with the language more generally, and the development went very
smoothly. Ultimately, that is how I remember that period of time when I first
wrote the C, C++ and Rust implementations: the Rust implementation went the
smoothest and was the most enjoyable. This project was perhaps the final
experience I needed to push me over the edge: I think for future programming I
will prefer to use Rust rather than C++ or C, provided there are no strong reasons not to
(such as available C/C++ libraries or existing C/C++ codebases). I really enjoy
writing Rust, I really enjoy the working environment of Rust, and it is (from
my very few personal experiences with it) just as performant if not more so
than C/C++.

&nbsp;

### Next Steps

I currently have a few plans for this repo. I have other things going on in my
life that are more important (including my job!) so whether or not I will get
around to all or even some of these remains to be seen, but I'll lay them out
here nonetheless.

1. Write a Python implementation of `cmp-tree`
    * I saw some methods in some Python libraries I used that would've made
      writing `cmp-tree` much simpler than it was in any of the languages I
      have written the program in so far, except for Bash. I am thinking that
      in the future, outside of the most basic of scripts, I would like to use
      Python for scripting instead of Bash. Bash is just a mess, and while I'm
      not a big Python fan, it has infinitely better support for arrays among
      many, many other advantages! I am also curious to see how a similar
      implementation would perform in Python. Given that my original Bash
      implementation was around as fast as my first `fork()` -> `exec(cmp)`
      implementation, maybe the Python performance won't be that far from the
      Rust implementation.
2. Make the Rust implementation multithreaded
    * I think the next thing for me to do in Rust (in general) is
      multithreading. Personally, I think multithreading might make the utility
      worse (since I think it will be less efficient in terms of actual CPU
      time) but I think when learning systems-level languages, you need to know
      how to multithread in the language you're learning, and as such I need to
      give it a go at some point. I also recognize that a multithreaded option,
      while perhaps slower when measured in total CPU time, would, on a normal,
      single-user computer, run faster in real-world time, and giving users the
      choice between a multithreaded execution and a single-threaded execution
      might be a good idea.
3. Talk about the perfomance of `cmp-tree` versus alternatives in the Project
   Report
    * While this project served (and continues to serve) mostly as a learning
      opportunity for me, I also very well might not have done quite as much
      work if I was unable to write a tool that performed the task faster than
      `diff -qr`. In the project report, I mention how I use `diff -qr` as a
      sort of goal, and when my implementations were 3 times slower than `diff
      -qr`, I knew there was some optimizing to be done. However, since I wrote
      my own file comparison function that uses `memcmp()` (or some
      equivalent), my program has been faster than `diff -qr` by a modest
      margin. 
4. Add support for files of all formats in Rust version
    * Currently the Rust implementation (and all the other implementations for
      that sake) only support directory trees comprised solely of directories
      and normal files. Pipes, links, and other forms of files that appear on
      Linux (the only OS I intend for this project to support) have no support
      yet. This, and the lack of a substantial testing suite are what prevent
      me from using `cmp-tree` for important tasks.
