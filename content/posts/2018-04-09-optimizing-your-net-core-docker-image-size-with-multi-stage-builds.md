---
layout: post
title: Optimizing your .NET Core Docker image size with multi-stage builds
description: An guide to minimising your .NET Core Docker image sizes.
---

My team at work has only recently started to utilise Docker for running our applications in both development and production, and as I've become more familiar with it, I've been quick to notice the commonly large sizes for even simple application images. With disk space as plentiful as it is these days, this is rarely an issue. However, there are still some scenarios where the size of your Docker images can play a large role:

* **Build/deploy times.** If you're using a continuous integration system (such as Travis CI) with Docker, it will probably be constantly downloading and uploading Docker images to build your application and then upload the resulting image for deployment. The larger your images, the more time will be spent waiting for these operations to complete.

* **Start-up time.** When running your application in production, the creation of new containers running your image (either for scale-out operations or deployment of new image versions) will often involve downloading the latest version of your application image. The longer the download takes, the longer it will be before the new container can be up and running.

For these reasons, I think it's still important to make the effort to keep your Docker images as small as possible. In the case of the .NET Core images we've been building at work recently, this is most easily accomplished using Docker's multi-stage build feature.

*This will be a fairly light introductory article on multi-stage builds and size optimisation. Scott Hanselman has a [great, more detailed post](https://www.hanselman.com/blog/OptimizingASPNETCoreDockerImageSizes.aspx) which also discusses building for ARM architectures. Peter Jausovec also has a post similar to mine but using [separate Dockerfiles instead of multi-stage builds](https://medium.com/@pjausovec/reducing-the-size-of-docker-images-4ea862d01555).*

### Building an un-optimised image

If we want to try and optimise our image size, we should first know what our baseline is to compare against! To do this, let's just create a default .NET Core console application by running:

{% highlight console%}
dotnet new console docker-size-test
{% endhighlight %}

That should create a simple "Hello World" console application in a new `docker-size-test` directory. If you want, you can double-check that everything is working by running `dotnet run` in that directory before proceeding.

Now, to set up our Docker image build for the application, create a new file called `Dockerfile` in the `docker-size-test` directory using your preferred text editor, containing the following:

{% highlight docker %}
FROM microsoft/dotnet:2.0-sdk
COPY . ./docker-size-test
WORKDIR /docker-size-test/
RUN dotnet build -c Release
ENTRYPOINT ["dotnet", "run", "-c", "Release", "--no-build"]
{% endhighlight %}

Since this is a fairly introductory article, I'll give a quick run-through of what all this means:

* `FROM microsoft/dotnet:2.0-sdk` specifies the "base image" to use to build our image. dotnet:2.0-sdk is an image published by Microsoft that contains everything we need to build and run a .NET Core 2.0 application.

* `COPY . ./docker-size-test` will copy all the contents of our current folder (the ".") into a new directory on the image, `docker-size-test`

* `WORKDIR /docker-size-test/` sets the directory we just created as the working directory for all following commands, so for every command we run it will be as if we're running it from inside the `docker-size-test` directory.

* `RUN dotnet build -c Release` builds our Hello World application in Release mode (using the `-c` "configuration" option)

* `ENTRYPOINT ["dotnet", "run", "-c", "Release", "--no-build"]` tells the Docker image what it should run whenever it starts up. We need to specify the Release configuration yet again here, otherwise the dotnet tool will assume we want it to run in Debug mode which will trigger a re-build of the application - which we don't want to do since it has already been built. The `--no-build` flag is used here just to help make sure that it is only using the build output that was created in the previous step; if for some reason it can't find our previous build output, the `--no-build` flag will cause it to fail because it can't generate a build on its own.

Now let's run the Docker build and name the image `docker-size-test` by running:

{% highlight console %}
docker build -t docker-size-test .
{% endhighlight %}

Once that's complete, you should be able to succesfully run the application using `docker run docker-size-test`.

So, now that the image is built, let's take a look at how big it is! Running `docker image ls` should give us the details of the image we just built:

{% highlight console %}
REPOSITORY       TAG     IMAGE ID      CREATED          SIZE
docker-size-test latest  4dc5be081c0a  28 minutes ago   1.75GB
{% endhighlight %}

That's a whole **1.75GB* image** for a Hello World application! Now, why might that be?

As I mentioned earlier, we use `microsoft/dotnet:2.0-sdk` as the base for our image, which contains all the appropriate dependencies to build and run our application. However, the build dependencies introduce quite a large size overhead, resulting in the large image size you see here.

Of course, when we run our image using `docker run docker-size-test`, we don't actually need the build dependencies to be present, do we? All we need is the .NET Core runtime to run our application. Wouldn't it be nice to use an image that only contains the runtime dependencies instead?

**1.75GB is the uncompressed image size. When downloading/uploading images, Dockers will compress them so it doesn't actually need to download that full size every time.*

### Introducing multi-stage builds

Multi-stage builds is a Docker feature that allows you to use multiple `FROM` statements in your Dockerfile - each `FROM` statement will define a new "stage". When defining multiple stages, Docker takes the last one and uses that as the output for your build.

What this lets us accomplish is to use the `dotnet:2.0-sdk` image to **build** our application, and then use another image - in this case `dotnet:2.0-runtime` which only contains the dependencies to **run** the application. As a result, the `dotnet:2.0-runtime` version of our application should be noticeably smaller.

For more information on multi-stage builds, see the [Docker documentation](https://docs.docker.com/develop/develop-images/multistage-build/#use-multi-stage-builds).

### Optimising our image

To utilise multi-stage builds to reduce our image size, we can update our Dockerfile to match the following:

{% highlight docker %}
FROM microsoft/dotnet:2.0-sdk AS build
COPY . ./docker-size-test
WORKDIR /docker-size-test/
RUN dotnet build -c Release -o output

FROM microsoft/dotnet:2.0-runtime AS runtime
COPY --from=build /docker-size-test/output .
ENTRYPOINT ["dotnet", "docker-size-test.dll"]
{% endhighlight %}

Going through the changes introduced here:

* The first line now specifies the `AS build` option, which gives this "stage" of the build a name. We need this so that we can refer back to the stage later in the file.

* I've added an extra parameter to the `dotnet build` command, `-o output` - this puts all of our build output into an output directory, which I mainly do so it's more obvious where in the directory structure it will reside.

* Next, we define a new build stage using `FROM microsoft/dotnet:2.0-runtime AS runtime`. The name here technically isn't necessary since we don't need to refer to this stage at any point, but I like having it for consistency's sake.

* `COPY --from=build /docker-size-test/output .` copies the contents of our output directory from the build stage into the root directory of our runtime stage. This is really the line that brings everything together, because it lets us use our built application in the runtime image without bringing across any of the build dependencies.

* Finally, `ENTRYPOINT ["dotnet", "docker-size-test.dll"]` - which has to be defined in our final stage, as this is the one that will be used for our final image - is a slight variation on the old `ENTRYPOINT` from our last Dockerfile. The `dotnet:2.0-runtime` image doesn't support commands like `dotnet run` without the full SDK, so instead we just tell it to run the DLL output that we got from our earlier dotnet build, which naturally has the same name as our application.

If you build your image again using `docker build -t docker-size-test .` and then run `docker run docker-size-test`, everything should still work as normal.

### The final comparison

Alright, now that we've built our new image using multi-stage builds, let's see how its size compares to before! Running `docker image ls` again reveals:

{% highlight console %}
REPOSITORY        TAG      IMAGE ID      CREATED           SIZE
docker-size-test  latest   b3a2d702b24a  17 minutes ago    219MB
{% endhighlight %}

**219MB!** That's nearly 90% of the entire image size cut down right there! (Still sounds a little large for Hello World perhaps, but such is the world of containerisation)

Although this is a fairly simplistic example, I think it does a great job illustrating the potential savings that can be made just by carefully choosing your runtime images and utilising Docker's multi-stage build features. So the next time you find yourself balking at the size of your Docker images, you just might be able to make some significant savings!
