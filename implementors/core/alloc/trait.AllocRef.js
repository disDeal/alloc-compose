(function() {var implementors = {};
implementors["alloc_compose"] = [{"text":"impl&lt;Primary, Fallback&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a> for <a class=\"struct\" href=\"alloc_compose/struct.FallbackAlloc.html\" title=\"struct alloc_compose::FallbackAlloc\">FallbackAlloc</a>&lt;Primary, Fallback&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Primary: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a> + <a class=\"trait\" href=\"alloc_compose/trait.Owns.html\" title=\"trait alloc_compose::Owns\">Owns</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Fallback: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a>,&nbsp;</span>","synthetic":false,"types":["alloc_compose::fallback_alloc::FallbackAlloc"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a> for <a class=\"struct\" href=\"alloc_compose/struct.NullAlloc.html\" title=\"struct alloc_compose::NullAlloc\">NullAlloc</a>","synthetic":false,"types":["alloc_compose::null_alloc::NullAlloc"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a>, C:&nbsp;<a class=\"trait\" href=\"alloc_compose/trait.CallbackRef.html\" title=\"trait alloc_compose::CallbackRef\">CallbackRef</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a> for <a class=\"struct\" href=\"alloc_compose/struct.Proxy.html\" title=\"struct alloc_compose::Proxy\">Proxy</a>&lt;A, C&gt;","synthetic":false,"types":["alloc_compose::proxy::Proxy"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a> for <a class=\"struct\" href=\"alloc_compose/struct.Region.html\" title=\"struct alloc_compose::Region\">Region</a>&lt;'_&gt;","synthetic":false,"types":["alloc_compose::region::Region"]},{"text":"impl&lt;Small, Large&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a> for <a class=\"struct\" href=\"alloc_compose/struct.SegregateAlloc.html\" title=\"struct alloc_compose::SegregateAlloc\">SegregateAlloc</a>&lt;Small, Large&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Small: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Large: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/alloc/trait.AllocRef.html\" title=\"trait core::alloc::AllocRef\">AllocRef</a>,&nbsp;</span>","synthetic":false,"types":["alloc_compose::segregate_alloc::SegregateAlloc"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()