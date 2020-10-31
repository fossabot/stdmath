(function() {var implementors = {};
implementors["stdmath"] = [{"text":"impl&lt;T, R, F&gt; RefUnwindSafe for Sigma&lt;T, R, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, R, F&gt; RefUnwindSafe for Product&lt;T, R, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl RefUnwindSafe for Method","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; RefUnwindSafe for FlippedIteratorOfTypes&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, F&gt; RefUnwindSafe for TransformedValue&lt;T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; RefUnwindSafe for Type&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;M, S&gt; RefUnwindSafe for ContextVal&lt;M, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;M: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; !RefUnwindSafe for Context&lt;R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;B, C, R&gt; RefUnwindSafe for ExcludedIterator&lt;B, C, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;C as Iterator&gt;::Item: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, R&gt; RefUnwindSafe for OverflowState&lt;T, R&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()