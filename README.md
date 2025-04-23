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

*Please refer to the `README.md` in `./rust/cmp-tree`*

&nbsp;

### Testing

This project includes a collection of directory trees that serve as input for
tests. To make things easier for developers, I have chosen to archive each test
as a `.tar.xz`. This means only one file is committed to the repo for each test
which saves those working on this project from having to commit perhaps dozens
and dozens of files for a single test that has a larger directory tree.

Because the tests are all archived however, it means they cannot be used for
testing until they have been extracted. I have 2 scripts to help with this
archiving process.

#### Extracting Tests

If you aren't modifying the tests but simply want to use the pre-existing tests
for testing, all you need to do is run the following command in the root of the
repo:

```
bash extract_tests.sh
```

This will extract every file whose file names ends in `.tar.xz` in the `tests/`
directory. If a directory of the same name as the `.tar.xz` file (minus its
extension) already exists, a warning will be printed and the `.tar.xz` will not
be extracted.

#### Archiving Tests

If you have modified an existing test or created a new test and want to commit
your changes, you can use the test archiving script. Run the following from the
root of the repo:

```
bash archive_tests.sh
```

This will archive every directory in `tests/` as a `.tar.xz`.

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
self-directed exploration and learning focused on systems-level languages and
development and scripting languages. In other words, it has allowed me to learn
more about systems-level development in general. Some of the questions or topics
I wanted to explore as various points throughout this project were:

1. The performance differences between Bash and C/C++/Rust
2. How certain objects (like `std::string` and `std::vector`) are implemented
   in C++
3. What it's like developing in C compared to more modern systems-level
   languages like C++/Rust
4. What it's like writing in a very modern language like Rust compared to C++
5. How to profile a program
6. How to program in Rust more generally
    * How do you efficiently read from a file?
    * How do you read file metadata?
    * How do you make a commandline application in Rust that takes commandline arguments?
    * How do you handle error reporting in your code?
    * How do you add tests to your project in Rust?
    * How do you write multithreaded code in Rust?

If you came here looking only for an executable that will allow you to compare
all the files in two directory trees, then simply build the Rust version of
`cmp-tree` and use that. If you want to hear a little more about what I learned
from this project, then please feel free to read the
[Project Report](#project-report) below.

&nbsp;

### Project Report

#### Part 1: Bash and C++

Originally I thought that Bash would be a suitable language for the simple tool
I wanted to make. The functionality of `cmp-tree` could be achieved in a Bash
script composed of 3 stages: (1) Run `find` on the first directory, then `find`
on the second directory. (2) Combine the results into one giant list, `sort`
it, and filter it so it only contains unique paths (using `uniq`). Finally, (3)
run `cmp` on `[first_directory]/[unique_find_path[i]]` and
`[second_directory]/[unique_find_path[i]]` to determine if file exists in both
trees and is identical.

It didn't take me long before I had a working Bash implementation, but it did
seem sort of slow. I looked online and found that `diff -qr [first_directory]
[second_directory]` already accomplished the behaviour I was trying to code in
`cmp-tree`. I compared the speeds between my Bash implementation and `diff
-qr`, and `diff -qr` was significantly faster (by about 3 times^?). I could
have simply stopped there and used `diff -qr` for my purpose, but I wondered if
I could make a program that could accomplish the task faster than `diff -qr`.
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

#### Part 2: C and the Perspective it Gave Me on C++

My biggest takeaway from writing the C implementation was that it took me
easily 4-6 times as long to write compared to the C++ implementation. This was
mostly because I wrote my own implementations of: (1) a C++ vector I called
`DynamicArray`, (2) a C++ string I called `String`, (3) basic functions that
would allow me to work with my `String`s as if they were paths, and (4) a
hand-written implementation of merge sort that worked on `DynamicArray`. This
was a great learning opportunity in many ways. I got to experience first hand
some of the frustrations with C that might've motivated C++ such as:

1. Having to hand-write implementations for things most programmers feel
   (perhaps rightfully so) entitled to such as vectors/dynamic arrays,
   decent strings, or even some sort of path object.
2. Proper support for generics so you can write one implementation of your
   dynamic array instead of one for each type of data your dynamic array needs
   to serve as a container for.
3. Proper support for generics so you can specify the type your dynamic array
   serves as a container for so that your compiler can warn you if you passed a
   dynamic array of ints to a function that only takes a dynamic array of
   strings, for instance.

I learned a lot about how strings and vectors might be implemented in C++, but
I also realized how much C can slow down development simply due to its lack of
ergonomic libraries (and features that enable them!). Writing the code for your
own strings or dynamic arrays is something I was spared in C++, but
unfortunately the more significant time saving came from not having to debug my
own implementations! Manual memory management is difficult, and I spent a good
chunk of time working to rid my string, dynamic array and path implementations
from bugs and memory errors.

This experience really helped me appreciate what C++ had going for it in
comparison to C. C++ spared me from having to write my own implementation of
strings, dynamic arrays, and so on, *and* made my code simpler to write and
read (thanks in part to the "object.verb()" syntax its classes allow for) *and*
saved me from many memory headaches because the various classes I used adhered
to the practice of RAII.

Writing `cmp-tree` in C wasn't all bad though. The experience also allowed me
to see at least 3 advantages C has over C++. The most conspicuous of which is
the fact that while you might have to implement something like dynamic arrays
from scratch in C, that implementation is likely bespoke and very well-suited
to the specific use case. This can mean performance gains and sometimes
increased confidence in the simple "objects" your larger project depends on. I
imagine some security-minded programs may want to have a lot of control over
the most fundamental "objects" in the project. C is also a simple language
compared to C++ (which is definitely more on the side of over-engineered).
Related to this, another disadvantage of C++ is that I find the style or
approach of codebases can vary a lot with C++, making it harder to get
comfortable with C++ projects.

Having completed my C implementation of `cmp-tree`, I definitely had a better
understanding of the pros and cons of C and C++. One thing I still needed to
do, however, was to compare the performance of my "built-from-the-ground-up" C
implementation and my C++ implementation. When I ran the speed tests, to my
shock, I once again found that my C implementation was on par with the Bash
implementation in terms of speed!

#### Part 3: Languages Can't Save You From Bad Design

Having Bash, C, and C++ implementations all performing about equal is obviously
a huge red flag. Something must be off about the greater design or the approach
I was implementing. At this point, both my C++ and C implementation would use
`opendir()` and `readdir()` to generate a list of relative paths representing
every file within a directory, including all the files in subdirectories. The
implementations would then do this again with the second directory, combine the
two file path lists, sort them, filter them to contain only unique entries.
Finally, they would go through the list of unique entries and `fork()` ->
`exec()` a `cmp` command on `[first_directory]/[unique_file_path[i]]` and
`[second_directory]/[unique_file_path[i]]`. This is exactly what the Bash
script does, and I opted for this approach for 2 reasons: (1) because it would
keep the general approach of each implementation of `cmp-tree` on equal
footing, allowing me to get a clearer sense of the performance of each
language, and (2) because it is simple and intuitive and unless performance is
unsatisfactory, clarity is a more important quality of code. Unfortunately,
this approach was leading to unsatisfactory performance, so it was time to make
some adjustments. I thought that the way the unique file list was generated was
probably not the bottleneck, but calling `fork()` so many times could be a part
of problem. After all, it's not free to just duplicate a process, and my
program heretofore was performing a lot of `fork`s.

Before making any changes to my C implementation though, I decided to use
`perf` to profile it.  I found that the C implementation was getting a lot of
L2 and L3 cache misses, and this seemed like an obvious bottleneck. I
remembered hearing recently that sometimes caches are specific to processes,
and if this was indeed the case then there would be no caching benefit if one
process cached memory that another process also wanted to read. In short,
forking so many times during execution could be slowing down the program in two
ways: (1) because forking and duplicating a process is a task that itself
increases the total amount of work my program needs to do to accomplish its
task and (2) the many forked processes may not benefit from the caching of
their siblings processes, preventing the program from benefitting (as much as
this program can) from caching.

My solution was to eliminate all forking by implementing a quick-and-dirty
function that would replace a `fork()` -> `exec("cmp
[first_directory]/[unique_file_path[i]]
[second_directory]/[unique_file_path[i]]")` call pair with my own function that
would simply read both files, comparing each corresponding byte, and if at any
point it found a difference, exiting and returning a status that indicates that
the two files are different. My first-draft of this function yielded no
significant performance gain which surprised me given that there should be
substantially less overhead - no forking, no commandline argument or flag
parsing (which `cmp` would be doing each time it got `exec`'d), none of the
extra functionality a more complete program like `cmp` might have. My
first-draft file-comparison function worked by reading a set number of bytes
from both files, and then looping through those bytes 1 by 1 and if the byte
from one file mismatched the corresponding byte from the other file, it exited
early. After some quick research, I found that using `memcmp()` to compare two
sets of bytes was possibly faster than doing this byte-by-byte comparison. I
changed the code to use `memcmp` and found my program was *substantially*
faster. It turns out that on a lot of CPU architectures, `memcmp()` has an
implementation written efficiently in assembly. For example, it could be
vectorized for your processor architecture, giving it huge speed gains over
whatever the compiler was able to do to my C/C++ code for comparing bytes. At
the time of writing, this switch from manually checking each byte to using
`memcmp()` represents the biggest speed improvement I've found so far.

#### Part 4: What About Multithreading?

Before making the switch from manually comparing bytes to using `memcmp()`, I
also tried multithreading the C implementation to see if that would make it
noticeably faster. It led to modest speed gains, but it also came at the cost
of using significantly more cores and with cache-miss downsides of its own. It
seemed to me like multithreading the program was a bit of a deal with the
devil: your program takes less real-world time to finish (on a typical,
multicore personal computer), but it takes more CPU time to do the same amount
of work. In other words, running on a very busy server or workstation, it
seemed to me like the multithreaded version could actually be slower (in
real-world time) due to: (1) the flat cost of dividing the work and passing it
onto threads, (2) the fact that on a busy CPU, it may be unlikely that any two
threads are both working at the same time, and (3) that there may also be more
cache misses because multiple threads are making non-adjacent, perhaps even
distant data accesses. Worse still, the multithreaded, non-`memcmp()` C
implementation (running on my idle laptop) was *still* multiple factors slower
than `diff -qr`! Using `memcmp()` represented a solution that simply made
better use of the underlying hardware. If the implementation of `memcmp()` was
vectorized, the code would make use of hardware capability that was just
underutilized in other implementations. Unlike with the multithreaded version
of `cmp-tree`, with `memcmp()` there were no considerations to take into
account - on a busy CPU or an idle CPU, using `memcmp()` represented
significant and straight forward performance gains. To top it all off, if it
made sense to do so, the benefits of using `memcmp()` could also be multiplied
by multithreading the program.

#### Part 5: Why Make a Rust Implementation?

At this point, I had C and C++ implementations that were around as fast if not
faster than `diff -qr`, so what's with the Rust version? I had 3 motivations
for making a Rust implementation of `cmp-tree`. First, at the time I was very
new to Rust and I thought this could be a good project for helping me
familiarize myself with what it was like to write code in Rust. For example,
this project represented my first chance to work with files and paths in Rust,
as well as serving as my first meaningful attempt to work with Rust's basic
data structures like `String`s and `Vec`s. More substantially, this was also
the first Rust project I had worked on that read files, took commandline
arguments, and was multithreaded. The second motivation for making a Rust
implementation was that this modestly-sized project would allow me to get a
sense of what the working environment for Rust would be like. What does
compilation look like? How is it creating documentation? How is it adding tests
to your code? Of course I could learn about this on a project of almost any
size, but this simple command line utility felt like a project that would
require an appropriate level of commitment. My final motivation for making a
Rust implementation of `cmp-tree` was to see how Rust compared to C++ and C in
terms of performance. Would all the memory safety come at the cost of slightly
worse performance? Would the promise of many zero-cost abstractions hold true?
I wanted to see for myself.

At this point, the Rust implementation `cmp-tree` is the fastest and the most
featureful, and I now feel like I can share my thoughts about Rust with respect
to those 3 initial motivations. When it came to writing the Rust implementation
of `cmp-tree`, one major (and surprising) takeaway I had was how quickly I was
able to get a working implementation done in Rust, given my lack of
familiarity. Even though it was my first real project working with files,
paths, `String`s and `Vec`s, I found all of the aforementioned to be very
comfortable to work with. I can honestly say that I preferred writing the code
in Rust to writing it in C++, and I can definitely say that I preferred writing
the code in Rust to writing it in C! I also really liked Rust's working
environment. Compilation is a breeze because I don't have to write a Makefile
or fiddle with CMake. Compilation is completely handled by `cargo` which
figures out what it needs to do from the Cargo.toml, and the `pub`, `use`, etc.
keywords. Lastly, when it came to performance testing, the non-multithreaded
Rust version was on par with C and C++ (even though at the time the C version
was multithreaded!) and the multithreaded Rust version is consistently the
fastest implementation in this repo.

In short, I had a great time writing the code in Rust. To me, it seemed like
there were lots of smart decisions that had been made with the data structures
I was using and with the language more generally, and the development went very
smoothly. Ultimately, that is how I remember that period of time when I first
wrote the C, C++ and Rust implementations: The C implementation was the most
arduous, the C++ implementation was fine, and the Rust implementation was hands
down the most enjoyable one to write.

This project was perhaps the final experience I needed to push me over the
edge: I think for future programming I will prefer to use Rust rather than C++
or C, provided there are no strong reasons not to (such as available C/C++
libraries or existing C/C++ codebases). I really enjoy writing Rust, I really
enjoy the working environment of Rust, and it is (from my very few personal
experiences with it) just as performant if not more so than C/C++.

#### Part 6: Continued Work: Adding Tests

Naturally, if I wanted to have a tool I could rely on, it would need to be
well-tested. That said, the impetus for making the first tests came from my
work on adding support to the Rust implementation for soft links. `diff -qr`
follows soft links and in my opinion, this is not adequate for supporting the
evaluation of soft links in directory trees. I've seen projects before which
have exists soft links that point to a location outside the project that it
assumes will exist. I'm not sure this is a wise way of doing things (as perhaps
it is better to have a script that creates said soft link, but only if the
destination exists and is the intended destination), but the the idea remained
that soft links should be evaluated based on their link path rather than on
whether what file, directory, etc. they point to is identical to what file,
directory, etc. their corresponding link points to. The first tests I wrote
were all about soft links and checked to make sure `cmp-tree` only evaluated
two soft links as identical if their target paths were identical, and on no
other condition. Whatever gets you there, I say. It's good that something
motivated me to start writing some tests.

Part of what held me back from writing tests for `cmp-tree` (other than the
fact that the project was aimed primarily at simple self-directed learning) was
that it wasn't clear to me how I should manage the input for the tests. If I'm
integration testing, my inclination was to have actual directory trees, the
paths to which we would feed as arguments to the program. The problem with this
is with enough tests, this can start to take up a lot of space on the
repository. Ultimately I did decide to go this route and if the space the tests
take up become too much, I'll consider other options. I was thinking that for
testing directory trees with large files, I could have a script in the test
that generates a giant file using something like `/dev/random` and then simply
`cp -ar`s the result into the other test directory so we know it is identical,
etc. Done this way, the repository can store a simple script rather than a
giant file, although this approach comes with its own risks too, of course.

Regardless, as of writing this, I have 10 fairly simple integration tests that
should catch any egregious bugs in the program. Next I plan to write some
unit-tests for the Rust functions that do all the heavy lifting to help me
catch less significant bugs. I also would like to figure out how to make tests
that allow me to test if `cmp-tree` can catch differences in file metadata. As
I understand it right now, `tar` doesn't preserve file metadata, nor does it
preserve permissions in the way I need it to (because it's impossible;
different computers have different users), and so I need to figure out a way to
create such tests first. One idea I'm considering that I've already hinted at
is having a script in each test directory that does any necessary setup work
for the test, including, perhaps setting the permissions and file metadata.

#### Part 7: Continued Work: Multithreading the Rust Implementation

After writing the non-multithreaded Rust implementation and achieving
performance at least noticeably better than `diff -qr`, I largely considered
this project complete. I accomplished my various goals for learning more about
systems-level development and in the process had built a tool for my purposes
better than what was already available on my system. That said, I knew that
there was room to grow for whatever implementation I decided would be the
primary. None of the implementations had any testing, most of them had at least
1 or 3 known (minor) bugs, most of them had no method for installing them for
all users on a computer (which would be nice if I actually intended to use this
tool at all), and only one of the implementations was multithreaded. Since I
wanted to get Rust experience and the Rust implementation was not only the most
fun to write but also the best performing, I decided I wanted to put some more
work into the Rust version of `cmp-tree`.

It didn't take me very long to multithread the Rust implementation and I can
confidently say that that was the best experience I have had multithreading a
program ever. As it goes with Rust, I had to figure out if Rust allowed for the
sort of behaviour I wanted to achieve (it did) and then how to actually
implement such behaviour in this language that I'm still new to. The good news
is that it didn't take much research, and once that was done, it was very, very
quick to implement. Most importantly however, I knew (unlike with my past
multithreading efforts in C) that my multithreaded Rust code wouldn't make
common multithreading mistakes like having two threads use overlapping slices
of an array as their input because Rust wouldn't have compiled that code.
Honestly, the first time the multithreaded Rust implementation compiled, it
passed all the tests I had at the time. That was only 10 tests, but coming from
my limited multithreading experience in C, I was expecting the majority of test
cases to fail because my code would have some off-by-one error that led to
erroneous data or something similar.

Time and time again in Rust I find that I have to pay an upfront cost of doing
research on how to accomplish the behaviour I want to achieve in a way that
Rust will allow, but that this pays off in the end by saving me from what could
be tonnes of time spent debugging a compiling but broken build. Having a
compiler catch a problem before your program ever even runs is one of the
reasons I like compiled languages in general. Rust just takes it a step further
than most compiled languages, and through its comprehensive rules catches more
problems. As I see it, I'm a bit surprised it's taken so long to get a language
like Rust, but nonetheless I'm grateful it exists now!


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
        * The Rust implementation of `cmp-tree` currently is multithreaded,
          although it does not provide an option to choose between
          multithreaded execution and single-threaded.
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
        * The Rust implementation of `cmp-tree` currently supports regular
          files, directories, and soft links.
