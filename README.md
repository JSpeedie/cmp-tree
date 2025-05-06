# cmp-tree

### Project Description

This repo contains multiple different implementations of the same program:
`cmp-tree`. The program functions as a sort of combination between `cmp` and
`tree`, allowing you to compare the full contents of two directories and find
any disparities.

#### Primary Implementation

Please note that if you are simply looking for a `cmp-tree` binary to run as a
fast, trustworthy tool, all you need to look at is the Rust implementation. The
Rust implementation was chosen as the primary implementation which means it is
the one that has received (and will continue to receive) more work so as to
have a well-functioning commandline tool at the end of this project, rather
than just a collection of incomplete implementations.

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

#### C, C++, Rust, and Python

*Please refer to the `README.md` in `./[language]/cmp-tree/`*

&nbsp;

### Testing

At the time of writing, only the Rust implementation of `cmp-tree` has tests
you can run to verify its correctness. This is because the Rust implementation
was chosen as the primary implementation.

This project includes a collection of test inputs (located in `./tests`) that
are used in the Rust tests but can also be used by the other implementations if
at some point it is decided to add tests for those. Keep in mind that the
number of test /inputs/ is not representative of how many /tests/ there are.
Each test input (and sometimes only parts of each test input) is reused
multiple times for various tests.

It is also worth explaining why all the tests appear in the repo as `.tar.xz`
files. This decision to store the test inputs in an archived form was done oo
make things easier for developers. Archiving a test input means committing only
one file per test, rather than possibly dozens of files (for larger directory
trees) per test.

Because the tests are all archived however, it means they cannot be used for
testing until they have been extracted. I have 2 scripts to help with this
archiving process.

#### Extracting Tests

If you aren't modifying the tests but simply want to use the pre-existing tests
for testing, all you need to do is run the following command in the root of
your local copy of the repo:

```
bash extract_tests.sh
```

This will extract every file whose file name ends in `.tar.xz` in the `tests/`
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
self-directed exploration and learning broadly focused on development and
performance differences between languages. Ultimately, it has been most useful
as an opportunity to learn more about systems-level development in general.
Some of the questions or topics I wanted to explore as various points
throughout this project were:

1. The performance differences between Bash and C/C++/Rust
2. Some details about how certain objects (like `std::string` and
   `std::vector`) are implemented in C++
3. What it's like developing in C compared to more modern systems-level
   languages like C++/Rust
4. What it's like writing in a very modern language like Rust compared to C++
5. The basics of program profiling
    * What insights can we get from `perf stat -d`?
    * What insights can we get from `perf record` + `perf report`?
    * What insights can we get from `callgrind`?
    * What insights can we get from `heaptrack`?
    * What insights can we get from `strace`?
6. How to program in Rust more generally
    * How do you efficiently read from a file?
    * How do you read file metadata?
    * How do you make a commandline application in Rust that takes commandline arguments?
    * How might you handle error reporting in your Rust program?
    * How do you add tests to your project in Rust?
    * How do you write multithreaded code in Rust?
7. How does developing in Python compare to limited and simple languages like
   Bash? How does it compare to involved and robust languages like C, C++ and
   Rust?

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

Regardless, as of writing this, I have 10 fairly simple integration tests and
16 unit tests that should catch any egregious bugs in the program. Next I plan
to write unit tests for some of the mid-level Rust functions as at the moment,
most of the unit testing thoroughly examines some of the highest level
functions. I also would like to figure out how to make tests that allow me to
test if `cmp-tree` can catch differences in file metadata. As I understand it
right now, `tar` doesn't preserve file metadata, nor does it preserve
permissions in the way I need it to (because it's impossible; different
computers have different users), and so I need to figure out a way to create
such tests first. One idea I'm considering that I've already hinted at is
having a script in each test directory that does any necessary setup work for
the test, including, perhaps setting the permissions and file metadata.

#### Part 7: Continued Work: Multithreading the Rust Implementation

After writing the non-multithreaded Rust implementation and achieving
performance at least noticeably better than `diff -qr`, I largely considered
this project complete. I accomplished my various goals for learning more about
systems-level development and in the process had built a tool for my purposes
better than what was already available on my system. That said, I knew that
there was room to grow for whatever implementation I decided would be the
primary. Most of the implementations had no testing and the only one that did
(the Rust implementation) had very limited testing (only 10 simple integration
tests!), most of them had at least 1 or 3 known (minor) bugs, most of them had
no method for installing them for all users on a computer (which would be nice
if I actually intended to use this tool at all), and only the C implementation
was multithreaded. Since I wanted to get Rust experience and the Rust
implementation was already the primary implementation, I decided to see if I
could multithread it as a means to improve performance.

It didn't take me very long to multithread the Rust implementation and I can
confidently say that doing so was the best experience I have had multithreading
a program ever. As it goes with Rust, I had to figure out if Rust allowed for
the sort of behaviour I wanted to achieve (it did) and then also figure out how
to actually implement such behaviour in this language that I'm still new to.
The good news is that it didn't take much research, and once that was done, it
was very, very quick to implement. Most importantly however, I knew (unlike
with my past multithreading efforts in C) that my multithreaded Rust code
wouldn't make common multithreading mistakes like having two threads use
overlapping slices of an array as their input because Rust wouldn't have
compiled such code. Honestly, the first time the multithreaded Rust
implementation compiled, it passed all the tests I had at the time. That was
only 10 integration tests, but coming from my limited multithreading experience
in C, I was expecting the majority of test cases to fail because my code would
have some off-by-one error that led to erroneous data or something similar.

Time and time again in Rust I find that I have to pay an upfront cost of doing
research on how to accomplish the behaviour I want to achieve in a way that
Rust will allow, but that this pays off in the end by saving me from what could
be tonnes of time spent debugging a compiling but broken build. Having a
compiler catch a problem before your program ever even runs is one of the
reasons I like compiled languages in general. Rust just takes it a step further
than most compiled languages, and through its comprehensive rules it catches
more problems. As I see it, I'm a bit surprised it's taken so long to get a
language like Rust, but nonetheless I'm grateful it exists now!

#### Part 8: Profiling for Further Improvements

One thing I wanted to learn more about through this project was profiling.
After changing my file comparison code to use `memcmp()` (or an equivalently
fast memory comparison option) and adding multithreading, I wasn't quite sure
how to improve the performance of the program next. I had one idea for
optimizing which is to have `cmp-tree` do its comparisons
directory-by-directory. By this I mean modifying `cmp-tree` so that when it is
going through a directory tree, it would get the list of files in the current
directory its in and compare those files *before* recursing further. This
differs from how `cmp-tree` worked at this point where it would recurse through
the full first and second directory trees to create two full file trees before
combining them into one giant unique file list which `cmp-tree` would finally
loop through, performing all the file comparisons. The thought behind this
alternative approach is that since it would access files to generate the file
list and then almost immediately actually compares the files, the disk accesses
the program performs over its runtime will be for data that is closer together
on the disk, speeding up the disk I/O which should be the slowest form of
significant data access in the program. I talk more about this alternative
approach in the Next Steps section, but for now it's worth noting (1) that it
was the next big optimization idea I had at the time, (2) that it would be a
lot of work, more than almost any other optimization I had implemented so far
or might reasonably be willing to do in the future, and (3) that I wasn't sure
this alternative approach to the program would actually make the program
execute faster.

Because I wasn't certain that this fundamentally different approach to the
program would yield performance gains, I decided I would look elsewhere for
optimization. Of course, this meant profiling. I first used `perf stat -d` to
check the performance counters. Branch misses were excellent, about 0.7%. L1
cache misses were not great, about 10%. L3 cache misses were excellent, about
0.3%. The standout piece of data however was the fact that given that
`cmp-tree` would auto-determine how many cores were on the machine it was
running on and create 1 thread per core, on my 16 core machine I was getting
~1.8 CPUs utilized. Better than single-threaded, but that is woefully poor CPU
usage by each thread on average (around 11.25% CPU utilization per thread).
It's hard for me to determine how much weight to put on this statistic though.
On one hand, it would make a lot of sense if `cmp-tree` was disk bound, in
which case CPU usage would be quite poor because the threads would all spend a
lot of time waiting for their file reading calls to return. On the other hand,
(as I'll cover below), I also had profiling info at this time that indicated
that reading files played a relatively small roll into total CPU time (~8%).
Maybe I'm misunderstanding the metrics, but to me that seemed to indicate that
file reading wasn't resulting in a lot of waiting *or* CPU work. Maybe I'm
failing to distinguish CPU utilization and CPU time, and maybe the call-graphs
I saw calculated their statistics based on how much time the CPU spent doing
actual work in those functions (and ignored how long the threads spent
waiting), and as such reported that reading files didn't take very long in
terms of CPU time. It's not clear to me at this point. Regardless, the point is
that CPU utilization was terrible, and that statistic alone could maybe guide
my following profiling efforts or future optimization.

I decided I maybe wanted some more info on cache performance so I ran a `perf
state -e` command asking for more information about cache misses and
references. It mentioned that about 8% of all cache refs missed. Something to
keep in mind.

Next I used `callgrind` to take a look at the call graph and which functions
were eating up the greatest portions of the total CPU time the program used. I
found that the majority of CPU time (~54%) was spent comparing chunks from
regular files, with only a small percentage (~8%) of run time spent reading the
files. This also surprised me since I thought this program could be really disk
heavy and that on the right input, disk I/O would contribute a significant
amount to total CPU time.

After checking the `callgrind` call graph, I profiled the memory usage of
`cmp-tree` using `heaptrack`. Again, to my surprise I found that `PathBuf`s and
`Path`s were using a lot more memory than I thought they would. Not as much as
`readdir`, but I already call `readdir` only once for every file across
both directory trees. Perhaps if I switched `cmp-tree` to the aforementioned
alternative, the directory-by-directory approach, I could simply print the output
of each comparison as it is performed and then reuse *one* `PathBuf`/`Path` per
thread, allowing me to remove the `first_path` and `second_path` members from
the `FullFileComparison` struct.

Finally, I also used `strace`, specifically with its `-r` flag to see how long
my program was spending in each syscall it called. At the time that's what I
thought the numbers added by the `-r` flag meant, but it turns out the `-r`
just tells you the gap in time between the previous syscall and the one it is
currently entering. These gaps can be helpful, but it's not what I was looking
for. This misunderstanding led to me seeing nothing particularly out of the
ordinary in the output of `strace` except for what I thought were a rare few
9x-slower-than-the-rest `statx()` calls. (In retrospect, it's not clear these
calls were slow at all! There may have just been a moderate delay between their
calling and the previously called syscall) That said, it did seem strange to me
that among the close-to-1000-lines of output my original `strace -r` call
generated, there were only 16 lines about `read()` when my program obviously
spends a lot of time reading from files (when the directory trees contain lots
of mostly-similar regular files, which the input I was giving it did). This
made me think `strace` was not going to be helpful for me. I would later find
out two things about calling `strace` that would change that opinion. First, I
found out that the `-r` flag was not giving me the information I thought it
was, and to actually get the amount of time each syscall took to complete, I
needed to use the `-T` flag instead. Next, I found out that in order to get
syscall info on all the threads created by my program I need to give `strace`
an additional flag: `-ff`. Running `strace -ff -T` gave me 95,000 lines of
output, **81,000** of which were `read()` calls! Now that seems more correct. I
also found one `futex()` call that took 0.81 seconds to complete. I didn't know
if that was an unreasonable length of time for that syscall, but I did know
that the average syscall in the program took closer to 0.000004 seconds. All
said and done, my main takeaways from using `strace` were (1) that perhaps I
can play with the read size of my read calls in order to improve performance.
81,000 out of 95,000 is a dominating presence from `read()`. Maybe that's fine,
but perhaps the startup and finishing costs of the `read()` syscall could be
adding up needlessly, and fewer, bigger reads could be a good idea. Further
still, maybe the first read could read quite a small number of bytes (to
quickly catch files that differ in the first few bytes) and increase the read
size for following read calls as we become increasing confident that the files
will match.

In short, I learned a lot about profiling and related tools such as `perf`,
`heaptrack`, and `strace`. The direction to take my optimization in next,
however, isn't perfectly clear. Perhaps the best idea for optimization I have
at the moment is pipelining. This was something I hadn't heard much about prior
to this profiling stint, but I recently learned that the producer-consumer
multithreading model is a common approach for pipelining the processing of many
files. Intuitively it could reduce the wait time each thread currently incurs
when they attempt to read from a file before comparing the read contents. It
would be a fair amount of work implementing it, but I think that's the next
direction I'll take this project. I'm looking forward to it! Hopefully it pays
off!

#### Part 9: Adding a Python Implementation

Adding a Python implementation of `cmp-tree` was something I wanted to do from
a very early point in this project. The main motivations stemmed from the fact
that Python, while perhaps not as heavily used as C or C++ in systems-level
work, is still often used for simple scripts supporting systems-level work.
This paired with the fact that (in my opinion) Python is a much more ergonomic
and built-out programming language than Bash made me want to practice writing
Python code so that I could start using Python instead of Bash for some of my
scripting work.

After I finished the Bash, C, C++ and Rust implementations, I got caught doing
extra work on each of them rather than quickly writing out a Bash
implementation. I multithreaded the C implementation, I added tests to the Rust
implementation, I multithreaded the Rust implementation, and just in general I
spent a lot of time working on the primary implementation so I could have a
reliable version of `cmp-tree` that I would actually be willing to use. The
point is, I got distracted with other work that I thought (and still think!)
was more important. But the time finally came and I wrote a Python
implementation.

It took me about half a work day to port the C++ code to Python (it helps to
have the project already designed and implemented somewhere else!). Honestly,
the main takeaway from this experience is that I really don't like Python haha.
Maybe I do the wrong kind of work, the kind Python isn't best suited to, but
man I just have a hard time seeing the circumstances when you would want to use
this language. Here's how I see it: if you want to use Python rather than Bash,
your script needs to be a little more complex than just a handful of lines
doing trivial work. However, the moment you're working in Python, you now have
to worry about the Python install situation on the machine your script is going
to be running in! You can't just rely on some instruction that your script be
run using `python3` and not `python`(2). Maybe your script uses stuff from
Python version 3.11 but the next computer you run your script on only have
Python 3.10. Maybe the next computer you run your script on doesn't even have
Python. Every Linux computer will have Bash so no worrying about installing
something prior to running your script.

Accounting for Python versions are not my main gripe with Python though. My
biggest concern about this language is that it just feels so unstable
programming in it. I think this is true of all interpreted languages, but man I
find it so uncomfortable programming in a non-compiled language. Obvious and
simple but nonetheless application-crashing bugs can be caught before your
program ever runs in compiled languages. In Python, it seems you just have to
run the code and find out later. Okay that's not entirely fair - Python does
seem to perform some checks right at the start of running your program, but
only so few, and a lot errors are only caught when the problematic code is hit
during run time. I don't doubt that there are Python development tools built to
solve this problem, but the fact that it isn't a part of the language is still
a huge downside in my eyes. I also recognize that in a well-built project, you
should have excellent test coverage and that if you have this, my complaint
seemingly becomes invalidated. In part I can agree with that, and at this point
it's true that I have yet to add tests to my Python implementation. However,
what continues to bother me is that without tests I feel like I have very, very
little trust in the implementation (even though I've obviously looked over
every single line in it at some point and they've all /seemed/ correct to me)
and that this increased danger has not come with any real benefit. It reminds
me of a contrast I see between Rust and C. Sure, maybe I have to spend more
time battling the compiler to get a compiling version of my implementation in
Rust. However, with Rust, once it compiles I find I'm often very close to the
finish line. With C, getting it to compile can sometimes feel like only the
half way mark. What I'm trying to say is that the qualities of Python that
allow for its supposed simplicity and speed of development are (in my opinion)
actually traps, and they don't reduce the time spent developing software, they
just offset some development and tack on interest. Python's indentation-scope
system and it's dynamic typing are two great examples of this. Prima facie,
super simple, flexible, ergonomic. Further down the road though, they obscure
what would otherwise be obvious mistakes in your code and in the case of the
indentation-scope system, (in my opinion) they make it possible to right some
truly hideous and hard to understand code.

I guess at the end of the day, there are no surprises to be found here. C is
the language I'm most comfortable in, and of course a C developer is going to
have these complaints about Python. It's funny because I can tell that I'm
currently going through a transition from being a C developer to being a Rust
developer and at the end of that process I'm going to have the same complaints
about C. "Sure C is harder than Python at first, but (in many ways) it's easier
than Python in the long run (if you're trying to develop quality software)!"
will get accompanied with "Sure Rust is harder than C at first, but (in many
ways) it's easier than C in the long run (if you're trying to develop quality
software)!"

&nbsp;

### Next Steps

I currently have a few plans for this repo. I have other things going on in my
life that are more important (including my job!) so whether or not I will get
around to all or even some of these remains to be seen, but I'll lay them out
here nonetheless.

#### General to the Repo:

1. Measure the performance of the Python implementation
    * I've just recently finished my implementation of `cmp-tree` in Python. At
      the moment I've been focused on testing and correctness since that's been
      a pain point with the Python implementation and obviously has to proceed
      any real speed tests. That said, I'm close to the point where I think I
      can start comparing its performance. It's likely I have a couple of
      simple mistakes in my Python code that are leading to non-negligible
      performance drops (perhaps there's a way to deliberately pass things by
      reference, for instance!) so maybe I can correct those too to give Python
      a fair chance since out of all the (real, not-just-for-scripting)
      languages I implemented `cmp-tree` in so far, it is the one I have the
      poorest understanding for writing performant code with it.
2. Talk about the perfomance of `cmp-tree` versus alternatives in the Project
   Report
    * While this project served (and continues to serve) mostly as a learning
      opportunity for me, I also very well might not have done quite as much
      work if I was unable to write a tool that performed the task faster than
      `diff -qr`. In the project report, I mention how I use `diff -qr` as a
      sort of goal, and when my implementations were 3 times slower than `diff
      -qr`, I knew there was some optimizing to be done. However, since I wrote
      my own file comparison function that uses `memcmp()` (or some
      equivalent), my program has been faster than `diff -qr` by a modest
      margin. It would be nice to see some graphs that show the progress trend
      of my speed of all my implementations.
3. Add more tests!
    * As it stands right now, I have a starter set of tests for the Rust
      implementation comprised of 10 integration tests and 16 unit tests that
      should help me catch any big bugs. That, and the fact that the program
      was written in Rust, gives me a decent sense of stability in the Rust
      implementation. That said, not only do I want to write unit tests for all
      the functions in the Rust version (a process I have only just begun), but
      I also would like to create more test inputs for the program in general.
      I want *many* tests that have files of the same relative path but with
      differing content, for starters, but also tests where relative paths
      point to files of different types (soft link vs regular file, for
      example) as I think this is currently completely untested.

#### Bash Implementation Specific:

1. Add support for the `-mpt` commandline arguments?
    * The Bash implementation is the least featureful and slowest
      implementation. It would be nice to add at least these commandline
      arguments so that it can offer the features every other implementation in
      this repo does, but I have my hesitations. First, I'm not sure adding the
      commandline argument passing will be clean, and second, I'm not sure
      adding the functionality of some of those flags will be quick and I'm
      honestly sort of ready to call it "done" on the Bash implementation. It
      is many factors slower than `diff -qr` (which, at the time of writing, is
      noticeably slower than my C, C++ and Rust implementations of `cmp-tree`)

#### C++ Implementation Specific:

1. Make the C++ implementation multithreaded.
    * I've never done any multithreading in C++ and this could be a great
      opportunity to explore things like Threading Building Blocks (TBB),
      `std::atomic<T>`, `std::lock_guard<std::mutex>`, and so on. I suspect my
      experience will be equally as arduous as it was multithreading the C
      implementation of `cmp-tree`, and not nearly as enjoyable as it was
      multithreading the Rust implementation, but it's worth giving a go. I'm
      way more likely to find myself multithreading C++ code in my career than
      I will be to find myself multithreading Rust code.
2. Add testing.
    * I'm new to C++, honestly. I'd say I only really started using it in 2024.
      Naturally, I don't have much or sometimes any experience with common
      elements in a standard toolchain with C++. This includes testing
      frameworks. I'd love to give `GoogleTest` a go!

#### C Implementation Specific:

1. Add a commandline option to the C version to limit the program to single
   threaded execution.
    * It's actually been quite a while between the time that I'm writing this
      idea for improving the C implementation and the last time I did any work
      on the C implementation, so it is possible I have already done this, but
      I really don't think I have. Anyway, I think it's a feature the program
      should have since multithreading is often a tradeoff.
2. Check for memory leaks.
    * Some of the functions I wrote as a part of my C implementation return
      values that need to be freed in some way or other by the caller. I'm
      afraid that in most of these cases, the functions that call these such
      functions do not free the return values. I need to take Valgrind to this
      program and see what it says.
3. Add testing.
    * I'll admit it: I don't have much experience writing tests for C projects.
      At this point, I'm starting to migrate away from C towards C++ and
      especially Rust, but this project could provide a good opportunity for me
      to get some experience with C testing frameworks.

#### Rust Implementation Specific:

1. Add a commandline option to the Rust version to limit the program to single
   thread execution.
    * As I mentioned in the Project Report, I think multithreading is very
      often a trade-off. You trade increased CPU time for less real-world,
      clock time. For this reason, I think it makes sense to have an option for
      users to force `cmp-tree` to run in single-threaded mode in case it is
      going to be run on a busy server, a battery powered device, or in some
      other situation where reducing the total amount of CPU work is more
      important than the real-world time savings to be gained from
      multithreading.
2. Add support for files of all formats in Rust version?
    * Currently the Rust implementation supports directory trees comprised
      solely of regular files, directories and soft links. Pipes, character
      devices, sockets, and other forms of files that appear on Linux (the only
      OS I intend for this project to support) have no support yet. I still
      have to decide if `cmp-tree` needs to support more than regular files,
      directories, and soft links, but maybe it should.
4. Optimize the Rust implementation further!
    * I'd love to make the primary implementation even faster! Right now I'm
      not super sure what to do to speed up the program further, although I
      have some ideas:
        * One idea for optimizing I have is to have `cmp-tree` go on a
          directory-by-directory basis. When going through a directory tree,
          `cmp-tree` would get the list of files in the current directory its
          in and compare those files *before* recursing further. This differs
          from how `cmp-tree` works now where it recurses through the full
          first and second directory trees to create two full file trees before
          combining them into one giant unique file list which `cmp-tree` then
          loops through, comparing files. The thought behind this alternative
          approach is that since it would access files to generate the file
          list and then almost immediately actually compares the files, the
          disk accesses the program performs over its runtime will be for data
          that is closer together on the disk, speeding up the disk I/O which
          should be the slowest form of significant data access in the program.
          One problem with this alternative approach is that it may not
          multithread as well since it won't have a complete list of all the
          files to compare which it could evenly divide up, and instead would
          have to blindly multithread on directories which may vary wildly in
          size. Of course the current multithreading implementation has a
          similar weakness with respect to file sizes. Currently, `cmp-tree`
          multithreads by giving each thread an equal number of files to
          compare from the complete file list. What would perhaps better even
          the workload would be to divide the list into chunks of equal size
          *in terms of bytes*, not file count so that one thread isn't stuck
          comparing four 3 gig files while another has to compare four 1 meg
          files. The alternative approach ultimately may be slower, and it
          would be a lot of work to implement, so I might prefer to find other
          optimizations first.
        * Perhaps if I switch to the "directory-by-directory" approach, I can
          print the output of each comparison as the comparison is performed.
          This could mean modifying only one `PathBuf`/`Path` per thread which
          could have performance payoffs. For example, since each thread would
          reuse a single `PathBuf`/`Path`, this might mean less heap
          allocations total since the single `PathBuf`/`Path` in each thread
          would grow to allocate longer paths, but could then remain that
          length for all future comparisons. In the current implementation
          where there is one `PathBuf` object per comparison, the program has
          to perform an allocation for every single Path. I'm not sure that
          this benefit outweighs some of the potential negatives of the
          "directory-by-directory" approach (e.g., that it likely won't
          multithread as well) but given that `PathBuf`s and `Path`s `.push()`
          and `.join()` methods are responsible for 28% of memory usage during
          the peak of memory usage, it's something to think about!
        * Another place for possible optimization is in
          `relative_files_in_tree()` where the file type of a directory entry
          is checked and after that, the file type data is essentially
          discarded. Later during execution, we check the file type of every
          file that gets compared (by a different means, but still getting
          essentially the same data). Perhaps we can save the program from that
          latter file check by storing the file type of a given file during
          `relative_files_in_tree()`. This sort of change would go against the
          structure of the `cmp-tree` as it exists now a little, but if I
          implement the "directory-by-directory" approach, this change would
          come about naturally, but I need not adopt that approach to implement
          this possible optimization.
        * Lastly, and most importantly, I'm starting to think my current
          multithreading model is the biggest current bottleneck to the
          program. I'm not certain how to interpret all the profiling data I
          recently collected, but here are the important pieces. First,
          `callgrind` indicates that ~8% of CPU time is spent on reading.
          That's not an enormous amount, but if I could take a huge bite out of
          it, it could save some meaningful time. Second, `perf stat -d`
          indicates that CPU utilization on my 16 core machine was on average,
          very poor at around 1.7 CPUs (out of a maximum of 16.0). All this,
          paired with the natural intuition that this program is likely disk
          bound (given that it interacts with so many files) leads me to think
          that switching my multithreading model to focus primarily on
          mitigating the slowness of disk operations could speed up the
          program. As such, I'm currently considering changing the
          multithreading to something in line with the producer-consumer model.
          I have 2 main reasons why I think this will speed things up:
            * First, CPU utilization is very poor with my current
              multithreading approach and I think there's a couple moments
              where things are spending a more time than they need to blocking.
              Right now, every thread handles its own file reading and its own
              file-chunk-comparing. I think a producer-consumer model where one
              thread reads a bunch of data and then gives it to a pool of
              threads dedicated to comparing data could be a lot faster. The
              reading thread can read the next set of data for all the threads
              *while* all the comparing threads are comparing the current set
              of data. This would (hopefully) effectively eliminate the dead
              time the threads currently experience where they wait for their
              read call to finish before comparing anything. This wait time
              could be a significant contributor to the poor CPU utilization I
              measured.
            * Second, disk access is most likely close to random in my current
              multithreading approach since as it stands right now, each thread
              takes 1 contiguous block of the total file list and works to
              complete all the comparisons in that block. This means that the
              first and last thread could be attempting to access files in
              their sections of the total comparison list at roughly the same
              time. These files are probably far apart on the actual file
              system and this could slow down disk I/O. Under a
              producer-consumer multithreading model where one reader thread
              reads the files in the list *in the order the files appear in the
              list*, read operations could be sped up simply because we are
              accessing disk in a more ordered way.
            * Third, in a multithreading model with one reader thread and a
              bunch of comparing threads, work could possibly be divided more
              evenly than under my current multithreading model. As it stands
              right now, `cmp-tree` divides the work between threads *evenly*
              in the sense that each thread has an equal number of comparisons
              to do. Of course, comparing identical 3 gig files is not an equal
              amount of work to comparing two identical 1 meg files. This is a
              weakness of my current multithreading model, and while not
              immediately resolved by a producer-consumer model, the reader
              thread could distribute an even *amount of bytes to compare* when
              doing regular file comparisons (by far the costliest comparisons
              to do). More evenly split work could lead to (by my guess) at
              best great speed gains (on particulary unequal directory trees,
              where files early in the list are very small, and files late in
              the list are ginormous) and at worst, next to negligible gains
              (when the total file size of all the files in each of the
              divided-up chunks of the file list are roughly the same). This
              problem of the unequal division of labour between threads can be
              solved within my current multithreading approach, but it is
              *easier* to resolve in a producer-consumer model.
            * In summary, the first advantage to the producer-consumer model
              would be that it would largely eliminate the time threads
              currently spend waiting for read calls (this time is instead
              replaced with the time it takes to communicate the
              data-to-be-compared between the reading thread and the comparing
              threads, which should hopefully be faster since it's all done in
              memory), and the second advantage is that all reading from the
              disk should be faster because the storage device will be accessed
              in a more efficient access pattern. The third advantage of the
              producer-consumer model is that it will be easier to divide the
              work very evenly between threads, making `cmp-tree` run much
              faster on lopsided directory trees.

#### Python Implementation Specific:

1. Add more testing to the Python version
    * The Python implementation is the implementation I feel least secure
      about. Python is not a (fully! *grinds teeth*) compiled language so who
      knows what simple errors I'll hit that my program just hasn't **run**
      into yet. Haha I feel like the only way to have any security at all in
      your Python code is to test it. I know this is true of other languages
      but it is *especially* true of Python, I feel. I have ported all the
      integration tests I wrote for the Rust version of `cmp-tree` to my
      testing framework for the Python implementation, but it would definitely
      be beneficial to port the existing unit-tests from the Rust
      implementation as well. As I said, I feel pretty insecure about the
      Python version so I should really write more tests too.
