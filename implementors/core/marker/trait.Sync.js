(function() {var implementors = {};
implementors["extendr_api"] = [{"text":"impl Sync for Bool","synthetic":true,"types":[]},{"text":"impl !Sync for ListIter","synthetic":true,"types":[]},{"text":"impl !Sync for PairlistIter","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Sync for PairlistTagIter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl !Sync for StrIter","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for Symbol&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for Character&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for Raw&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Lang&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;NV&gt; Sync for Pairlist&lt;NV&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NV: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for List&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for Expr&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;P, NV&gt; Sync for Env&lt;P, NV&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;NV: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;F, B, E&gt; Sync for Func&lt;F, B, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;C, E, V&gt; Sync for Promise&lt;C, E, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for Primitive&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, D&gt; Sync for RArray&lt;T, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl !Sync for Error","synthetic":true,"types":[]},{"text":"impl !Sync for Robj","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()