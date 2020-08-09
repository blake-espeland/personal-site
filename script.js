var x = 0;
function scrollhandle() {
  //document.getElementById("name").style.marginleft = x += 1;
  console.log(x);
}

function scrollToAnchor(aid) {
  var aTag = $("a[name='" + aid + "']");
  $("html,body").animate({ scrollTop: aTag.offset().top }, "slow");
}

$(function () {
  $("#name").hide();
  $(".hbutton").hide();
  $(".media").hide();
  $("#name").fadeIn("slow", function () {
    $(".hbutton").fadeIn("slow", function () {
      $(".media").fadeIn("slow");
    });
  });
});
