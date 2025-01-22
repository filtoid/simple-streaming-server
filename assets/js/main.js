var filter = "";
$(document).ready(function(){
    refresh();
});

function refresh() {
    var url = "api/files?" + encodeURIComponent(filter);
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status !== "ok"){
                $('#error').html(ret.message);
                return;
            }
            var folder_output = refresh_go_back();
            for(var i=0;i<ret.folders.length;i++){
                let fold_name = ret.folders[i].replaceAll("\"", "").replaceAll("'", "");
                folder_output += "<div class='folder_button' onclick='add_filter(\"" + fold_name + "\")'>" + fold_name + "</div>";
            }
            $('#folders').html(folder_output);

            var file_output = "";
            for(var i=0;i<ret.files.length;i++){
                let fn = ret.files[i].replaceAll("\"", "").replaceAll("'", "");
                if(filter !== ""){
                    fn = filter + "/" + fn;
                }
                let video_str = encodeURIComponent(fn).replaceAll("%22", "");
                console.log(video_str);
                file_output += "<a href='/video?" + video_str + "'>" + fn + "</a><br>";
            }
            $('#files').html(file_output);

            ;
        },
        error: function(ret){
            console.log("ERROR creating new collection");
            console.log(ret);
        }
    })
}

function refresh_go_back() {
    if(filter !== "") {
        return "<div class='folder_button' onclick='remove_filter()'>Back</div>";
    } else {
        return "";
    }
}

function remove_filter() {
    var f_comp = filter.split("/");
    var new_filt = "";
    for(var i=0;i<f_comp.length-1;i++){
        new_filt += f_comp[i];
        if(i!==f_comp.length-2) {
            new_filt += "/"
        }
    }
    filter = new_filt;
    refresh();
}

function add_filter(filt) {
    filter += "/" + filt;
    refresh();
}