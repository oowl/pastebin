var a_idx = 0;
jQuery(document).ready(function($) {
  $("body").click(function(e) {
    var a=['#1793d3','#f57421','#c70036','purple','#10baee','#294172','#87cf3e','#73ba25','#fb3920','#8d86bb','#34be5b'];
    var $i = $("<span/>").text("+1s");
    a_idx = (a_idx + 1) % a.length;
    var x = e.pageX,
    y = e.pageY;
    $i.css({
      "z-index": 999,
      "top": y - 20,
      "left": x,
      "position": "absolute",
      "font-weight": "bold",
      "font-size":"17px",
      "color": a[a_idx]
    });
    $("body").append($i);
    $i.animate({
      "top": y - 180,
      "opacity": 0
    },
    1500,
    function() {
      $i.remove();
    });
    a_idx = (a_idx + 1) % a.length;
  });
});