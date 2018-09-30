jQuery(document).ready(function ($) {
    let a_idx = 0;
    $("body").click(function (e) {
        var a = ['#1793d3', '#f57421', '#c70036', 'purple', '#10baee', '#294172', '#87cf3e', '#73ba25', '#fb3920', '#8d86bb', '#34be5b'];
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
            "font-size": "17px",
            "color": a[a_idx]
        });
        $("body").append($i);
        $i.animate({
                "top": y - 180,
                "opacity": 0
            },
            1500,
            function () {
                $i.remove();
            });
        a_idx = (a_idx + 1) % a.length;
    });


    let copy_button = new ClipboardJS('.copy', {
        target: function (trigger) {
            return trigger.nextElementSibling;
        }
    });

    let show_copy_res = async (...rest) => {
        Array.prototype.slice.call(rest,1);
        let yesno = ['green', 'red'];
        let $info = $("<span/>");
        $info.css({
            "top": "10px",
            "right": "40px",
            "position": "absolute",
            "font-weight": "bold",
            "font-size": "14px",
            "display": "none"
        });
        $("pre").append($info);
        if (rest[0]) {
            $info.text(rest[1]);
            $info.css('color', "#55a96f");
            let X=$('.flyowl').offset().left;
            $(".flyowl").animate({'left':screen.width-X},'slow',function(e){
                $(".flyowl").css('left',-X).animate({'left':0},'fast')
            })
//             setTimeout(function(){
//                $(".flyowl").css('left','0px') 
//             },600)
        } else {
            $info.text('Copy error!');
            $info.css('color', yesno[1]);
        }
        $info.fadeIn(300);
//         $info.slideDown(400);
//         $info.slideUp(1000);
        setTimeout(function(){
            $info.fadeOut(900);
            setTimeout(function(){
                $info.remove();
            },1000)
        },900);
        
    }

    copy_button.on('success', (e) => {
        e.clearSelection();
        show_copy_res(true, 'Copied!');
    });

    copy_button.on('error', (e) => {
        show_copy_res(false, fallbackMessage(e.action));
    });

});