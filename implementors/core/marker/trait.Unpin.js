(function() {var implementors = {};
implementors["alloc_compose"] = [{"text":"impl&lt;Primary, Fallback&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/struct.FallbackAlloc.html\" title=\"struct alloc_compose::FallbackAlloc\">FallbackAlloc</a>&lt;Primary, Fallback&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Fallback: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Primary: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["alloc_compose::fallback_alloc::FallbackAlloc"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/struct.NullAlloc.html\" title=\"struct alloc_compose::NullAlloc\">NullAlloc</a>","synthetic":true,"types":["alloc_compose::null_alloc::NullAlloc"]},{"text":"impl&lt;A, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/struct.Proxy.html\" title=\"struct alloc_compose::Proxy\">Proxy</a>&lt;A, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["alloc_compose::proxy::Proxy"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/struct.Region.html\" title=\"struct alloc_compose::Region\">Region</a>&lt;'a&gt;","synthetic":true,"types":["alloc_compose::region::Region"]},{"text":"impl&lt;Small, Large&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/struct.SegregateAlloc.html\" title=\"struct alloc_compose::SegregateAlloc\">SegregateAlloc</a>&lt;Small, Large&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Large: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Small: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["alloc_compose::segregate_alloc::SegregateAlloc"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/stats/struct.Counter.html\" title=\"struct alloc_compose::stats::Counter\">Counter</a>","synthetic":true,"types":["alloc_compose::stats::Counter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/stats/struct.AtomicCounter.html\" title=\"struct alloc_compose::stats::AtomicCounter\">AtomicCounter</a>","synthetic":true,"types":["alloc_compose::stats::AtomicCounter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/stats/struct.FilteredCounter.html\" title=\"struct alloc_compose::stats::FilteredCounter\">FilteredCounter</a>","synthetic":true,"types":["alloc_compose::stats::FilteredCounter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"alloc_compose/stats/struct.FilteredAtomicCounter.html\" title=\"struct alloc_compose::stats::FilteredAtomicCounter\">FilteredAtomicCounter</a>","synthetic":true,"types":["alloc_compose::stats::FilteredAtomicCounter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"enum\" href=\"alloc_compose/stats/enum.AllocInitFilter.html\" title=\"enum alloc_compose::stats::AllocInitFilter\">AllocInitFilter</a>","synthetic":true,"types":["alloc_compose::stats::AllocInitFilter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"enum\" href=\"alloc_compose/stats/enum.ReallocPlacementFilter.html\" title=\"enum alloc_compose::stats::ReallocPlacementFilter\">ReallocPlacementFilter</a>","synthetic":true,"types":["alloc_compose::stats::ReallocPlacementFilter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"enum\" href=\"alloc_compose/stats/enum.ResultFilter.html\" title=\"enum alloc_compose::stats::ResultFilter\">ResultFilter</a>","synthetic":true,"types":["alloc_compose::stats::ResultFilter"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()