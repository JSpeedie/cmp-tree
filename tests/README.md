## Generating All the Test Inputs

You can simply run the script provided in this directory:

```bash
./generate-all-test-inputs.sh
```

### ImageMagick Ubuntu/Linux Mint

I don't have a lot of experience with Ubuntu/Linux Mint, but it seems like
when you install ImageMagick on these distros the command you use to invoke it
differs from the command you would use on Arch Linux. On Arch, you'd use `magick`,
but on Ubuntu/Linux Mint, you'd use `convert`. To save you from manually modifying
all the scripts, you can simply create a soft link like so:

#### 1. Find out where you `convert` executable is located

```
which convert
```
```
/usr/bin/convert
```

#### 2. Create the necessary soft link

```
sudo ln -s /usr/bin/convert /usr/bin/magick
```

This will create a soft link located at `/usr/bin/magick` that redirects to
`/usr/bin/convert`.

#### 3. Generate the tests!

At this point everything should work again!
