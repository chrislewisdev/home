---
layout: post
title: React without npm, Babel, or webpack
description: A look at how to get started using React without all the requirements of package managers and bundlers.
---

*This 2017 blog post is a little out of date now, and in fact React now have their own getting started guide that does not require any build tooling: ["Add React to a Website"](https://reactjs.org/docs/add-react-to-a-website.html). That said, this post may still be useful for understanding how all these components relate in a React app, so feel free to give it a read anyway ;)*

When I start learning a new tool or framework, I like to try and keep it as isolated from other concepts as possible, so I can focus just on that one thing and nothing else. It avoids frustration with auxiliary tooling and helps me make sure I understand exactly what is happening in any given example.

In the case of learning React, I wanted to have a full working example in a plain HTML/Javascript file that I can simply open in any browser- no development webserver or anything involved.

Sounds simple enough, right? Well, it *is* once you know how, but the React documentation makes it a little difficult to tell how best to do this. To quote:

> While React [can be used without a build pipeline](https://facebook.github.io/react/docs/react-without-es6.html), we recommend setting it up so you can be more productive. A modern build pipeline typically consists of:
>
> A **package manager**, such as [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/). It lets you take advantage of a vast ecosystem of third-party packages, and easily install or update them.
>
> A **bundler**, such as [webpack](https://webpack.js.org/) or [Browserify](http://browserify.org/). It lets you write modular code and bundle it together into small packages to optimize load time.
>
> A **compiler** such as [Babel](http://babeljs.io/). It lets you write modern JavaScript code that still works in older browsers.

Now don't get me wrong, those are all good tools to use, but that's a lot of things I don't want for just a simple sample file! My aim with this post is to collate all the necessary information to avoid all those things altogether.

Let's look at how we can avoid the need for all of these things by converting [this CodePen sample](https://codepen.io/anon/pen/brvpjG?editors=0010) into something we can run in a plain HTML file:

{% highlight jsx %}
{% raw %}
class Greetings extends React.Component
{
  render()
  {
    return (
      <h1>Greetings, {this.props.name}!</h1>
    );
  }
}

ReactDOM.render(
  <Greetings name="Chris" />,
  document.getElementById('root')
);
{% endraw %}
{% endhighlight %}

*Note: I won't be explaining how React actually works in this post, just how to translate from most React samples into easily runnable code. If you want to know how this sample actually works, consult the [Quick Start Guide](https://facebook.github.io/react/docs/hello-world.html) for React.*

## Avoiding npm

In order to avoid setting up npm for the sake of a *single file*, we can reference a CDN for the two main React libraries, react and react-dom:

{% highlight html %}
{% raw %}
<script src="https://unpkg.com/react@15/dist/react.js"></script>
<script src="https://unpkg.com/react-dom@15/dist/react-dom.js"></script>
{% endraw %}
{% endhighlight %}

Alternatively, you can manually save those two JS files into your workspace and reference them locally. (it's no big concern for a sample file, but I did on one occasion find the CDN had been updated with a broken file, which broke my sample project)

## Avoiding webpack

Since all our work will be in a single file, we won't run into any need for webpack in this sample as is. Even if your samples expand and you want to break stuff into several files, you can just include them all in index.html with script tags- there's honestly no need to introduce a build system for only a handful of files!

## Avoiding Babel

This is where we'll actually have to make some modifications to the JS code to get it running. Most React samples use JSX, aka the bits that mix in HTML syntax inside JavaScript:

{% highlight jsx %}
{% raw %}
return (
  <h1>Greetings, {this.props.name}!</h1>
);
{% endraw %}
{% endhighlight %}

If you try and run anything like that locally, you'll just get a boring old syntax error message in your console.

However, that JSX code is actually just syntactic sugar for the following:

{% highlight javascript %}
{% raw %}
return React.createElement('h1', null, 'Greetings, ' + this.props.name + '!');
{% endraw %}
{% endhighlight %}

Any JSX block can be converted into a call to React.createElement with three arguments:

* The type of element to create. Since we're just creating a standard H1 element, as opposed to a custom React component, we specify it with a plain old string.

* A properties object which specifies any property values to be set on that element. Since we're only creating a plain H1 element, null will suffice, but we could use this to specify a CSS class on it, for example.

* Any child elements to place in it. For plaintext contents like we're using here, a string is all you need, but I'll give an example at the bottom where we pass in additional React components.

And that's it! So, as for the other JSX block in our code:

{% highlight jsx %}
{% raw %}
<Greetings name="Chris" />
{% endraw %}
{% endhighlight %}

This can just be expressed as:

{% highlight javascript %}
{% raw %}
React.createElement(Greetings, { name : 'Chris' })
{% endraw %}
{% endhighlight %}

This time we are actually providing some properties to the element, and since we don't need to give it any child elements, we're omitting the third parameter altogether.

## The End Result

That's honestly all it takes to get React working without all the guff of package managers and build systems. All together, it looks like this:

{% highlight html %}
{% raw %}
<html>
  <head>
    <title>React Hello World</title>
    <script src="https://unpkg.com/react@15/dist/react.js"></script>
    <script src="https://unpkg.com/react-dom@15/dist/react-dom.js"></script>
  </head>
  <body>
    <div id="root"></div>

    <script>
      window.onload = function()
      {
        class Greetings extends React.Component
        {
          render()
          {
            return React.createElement('h1', null, 'Greetings, ' + this.props.name + '!');
          }
        }

        ReactDOM.render(
          React.createElement(Greetings, { name : 'Chris' }),
          document.getElementById('root')
        );
      };
    </script>
  </body>
</html>
{% endraw %}
{% endhighlight %}

Give that a shot on your local machine and it should work right out of the box!

That should be all the setup you need to start toying with React examples whilst knowing there's no unknown magic occurring in the background (at least, outside of React itself). Start working through the [Quick Start Guide](https://facebook.github.io/react/docs/hello-world.html), and enjoy!

## Additional Notes

### Passing multiple children to React.createElement

As I mentioned earlier, you can provide as many children to an element created with React.createElement as you like. Suppose instead of one Greeting you wanted to nest multiple Greetings within a div tag:

{% highlight javascript %}
{% raw %}
React.createElement('div', null,
  React.createElement(Greetings, { name : 'Chris' }),
  React.createElement(Greetings, { name : 'Ming' }),
  React.createElement(Greetings, { name : 'Joe' })
)
{% endraw %}
{% endhighlight %}

Simple! And honestly not too ugly if you space out the function calls sensibly.

### I want all the extra tooling but don't want to set it up myself

The React website has a ["Create a new React App"](https://reactjs.org/docs/create-a-new-react-app.html) that will probably point you in the right direction!

*Update 11/10/2018: Originally I was referring to the [Fountain](https://github.com/FountainJS/generator-fountain-react) project, but this doesn't seem to be recently maintained and the original website domain no longer belongs to them. Use with caution.*

### Avoiding JS classes

The above examples should work fine in any modern browser, but if you're conscious of avoiding all potentially-unsupported ES6 code including classes, you can just replace the definition of "Greetings" with the following:

{% highlight javascript %}
{% raw %}
var Greetings = React.createClass(
{
  render: function ()
  {
    return React.createElement('h1', null, 'Greetings, ' + this.props.name + '!');
  }
});
{% endraw %}
{% endhighlight %}

React seems to be discouraging the use of React.createClass for the future, but for now you can use it to define React components with object definitions instead of straight-up classes.

For more details on avoiding ES6 code in your samples, consult [React Without ES6](https://facebook.github.io/react/docs/react-without-es6.html)*.*

*Sources: [React Without ES6](https://facebook.github.io/react/docs/react-without-es6.html), [React Without JSX](https://facebook.github.io/react/docs/react-without-jsx.html), [React Installation (Add React to an Existing App)](https://reactjs.org/docs/add-react-to-an-existing-app.html)*
