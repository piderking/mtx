// Export Just the HTML PARTS of compiled (heading will be done by main.rs)
/*
       <div class="notebook">
           <div class="title">
               <h2>Tidfsdf essdf sdf sdfdf 67</h2>
           </div>
       </div>
       <div class="notebook">
           <div class="container">
               <div class="gutter">
                   Example:
               </div>
               <div class="problem">:w

                   <div class="snippet mtx">let f(x) = x^2 + 2x + 1</div>
                   <div class="snippet math">
                       <!-- SVG injected here -->
                       <svg>...</svg>
                   </div>
                   <div class="snippet python">
                       <pre><code>def f(x): return x**2 + 2*x + 1</code></pre>
                   </div>
               </div>

           </div>
       </div>


       <!--EXAMPLE-->
      sdfsd
      s
   </div>

* */

pub mod html;

use crate::ast::Statement;
