import * as wasm from "automata";

var $;
$ = require('jquery');

$(document).ready(function(){
    var automata = $.parseJSON(wasm.init());
    var update_table_caption = function(){
        $("#caption").html("Type: " +  automata["automata_type"]);
    }

    var update_alphabet_display = function(){
        var alphabet = automata["alphabet"];
        $("table#display-table>thead").html("");
        $("table#display-table>thead").append("<th>State</th>");
        $("table#add-state-table>tbody").html("");
        var value = '<tr><td><input type="text" id="state" size="5" placeholder="q0"></td>';
        for (var i  = 0; i < alphabet.length; i++){
            $("table#display-table>thead").append("<th>" + alphabet.charAt(i) + "</th>");
            value += '<td><input type="text" class="connection" id="connection_' + i + '" size="5" placeholder="q1"></td>';
        }
        if(automata["automata_type"] == "AutomataEClosure"){
         $("table#display-table>thead").append("<th>EMove</th>");
        }
        value += '<td><input type="text" class="connection" id="connection_' + alphabet.length + '" size="5" placeholder="q1"><td></tr>';
        $("table#add-state-table>tbody").append(value);

        $("table#add-state-table>tbody").trigger('resize');

        $.each($("table#display-table>thead").children('th'), function(i, child){
            if(i == 0){
                $("#state").css("width", ($(child).outerWidth(true) - 2) + "px");
            }
            else {
                $("#connection_"+(i-1)).css("width", ($(child).outerWidth(true) - 2) + "px");
            }
        });
    }   

    var update_states_display = function() {
        var states = automata["states_connections"];
        var alphabet = automata["alphabet"];
        $("table#display-table>tbody").html("");
        $.each(states, function(index, val) {
            if(automata["automata_type"] == "AutomataEClosure" || automata["automata_type"] == "AutomataNFA"){
                 var row = "<tr>";

                 var state_name = val[0]["name"];
                 row+= "<td>";
                 if (automata["start_state"] != undefined && automata["start_state"]["name"] ==  state_name){
                     row+="->";
                 }

                 $.each(automata["end_states"], function(index, value){
                     if(value["name"] == state_name){
                         row+="*";
                         return;
                     }
                 });

                 row+= state_name + "</td>";

                 var connections = [];

                 for (var i  = 0; i < alphabet.length; i++){//for_each value connection
                     connections.push([]);
                     for (var j = 0; j < val[1].length; j++){
                         if (val[1][j]["Value"] === undefined)
                             continue;
                         if (val[1][j]["Value"][1] == alphabet.charAt(i)){
                             connections[i].push(val[1][j]["Value"][0]["name"]);
                         }
                     }
                 }


                 //last one is emove
                 if(automata["automata_type"] == "AutomataEClosure"){   
                     connections.push([]);
                     for (var j = 0; j < val[1].length; j++){
                         if (val[1][j]["EMove"] !== undefined){
                             connections[connections.length-1].push(val[1][j]["EMove"]["name"]);
                         }
                     }
                 }

                 for(var i = 0; i < connections.length; i++){
                     var current_row = "";
                     for(var j = 0; j < connections[i].length; j++){
                         current_row+= connections[i][j] + ", ";
                     }
                     if (connections[i].length == 0){
                         row+= "<td>-</td>";
                     }
                     else{
                      row += "<td>" + current_row.substr(0, current_row.length-2) + "</td>";
                     }
                 }
             

                 row+="</tr>";

                 $("table#display-table>tbody").append(row);
            }
            else {//dfa display            
                var get_name = function(compound_state){
                    var init_value = "[";
                    $.each(compound_state["states"], function(i, each_val){ 
                        init_value += each_val["name"]+", ";
                    });
                    return init_value.substr(0, init_value.length-2) +']';
                };

                var row = "<tr>";;
                var state_name = get_name(val[0]);
                 row+= "<td>";
                 if (automata["start_state"] != undefined && get_name(automata["start_state"]) ==  state_name){
                     row+="->";
                 }

                 $.each(automata["end_states"], function(index, value){
                     if(get_name(value) == state_name){
                         row+="*";
                         return;
                     }
                 });

                 row+= state_name + "</td>";

                 var connections = [];

                 for (var i  = 0; i < alphabet.length; i++){//for_each value connection
                     connections.push([]);
                     for (var j = 0; j < val[1].length; j++){
                         console.log(val[1][j]);
                         if (val[1][j][1] === undefined)
                             continue;
                         if (val[1][j][1] == alphabet.charAt(i)){
                             connections[i].push(get_name(val[1][j][0]));
                         }
                     }
                 }


                 for(var i = 0; i < connections.length; i++){
                     var current_row = "";
                     for(var j = 0; j < connections[i].length; j++){
                         current_row+= connections[i][j] + ", ";
                     }
                     if (connections[i].length == 0){
                         row+= "<td>-</td>";
                     }
                     else{
                      row += "<td>" + current_row.substr(0, current_row.length-2) + "</td>";
                     }
                 }
             

                 row+="</tr>";

                 $("table#display-table>tbody").append(row);
            }
        });   
    }

    var update_transition_button_text = function(){
        var state = automata["automata_type"];
        var button = $("#transition-button");
        if(state == "AutomataEClosure"){
            button.text("Transition to AutomataNfa");
        }
        else if(state == "AutomataNFA"){
            button.text("Transition to AutomataDfa");
        }
        else if(state == "AutomataDFA"){
            button.text("Transition to AutomataDfaMinimized");
        }
        else if(state == "AutomataMinimizedDFA"){
            button.remove();
        }
    };

    var update_transition_button_display = function(){
        if(automata["start_state"] != null && automata["end_states"] != ""){
            $("#transition-button").removeAttr("disabled");
        }
    }

    var update_start_end_states_display = function(){
        if(automata["states_connections"].length!=0 && automata["automata_type"]=="AutomataEClosure"){
            $(".start-end-states").css("visibility", "visible");
        }
        else {
            $(".start-end-states").css("visibility", "hidden");
        }
    }
    var transition = function(){
        var state = automata["automata_type"];
        if(state == "AutomataEClosure"){
            automata = $.parseJSON(wasm.transition_to_nfa(automata));
        }
        else if(state == "AutomataNFA"){
            automata =  $.parseJSON(wasm.transition_to_dfa(automata));
        }
        else if(state == "AutomataDFA"){
            automata =  $.parseJSON(wasm.transition_to_dfa_minimized(automata));
        }
        update_transition_button_text();
        update_table_caption();
        update_states_display();
        update_alphabet_display();
        update_start_end_states_display();
        $("#add-state-section").remove();
    };
    $("#add-state-button").click(function(){
        //add states
        var value = $("#state").val();
        if(value == "" || value == undefined){
            return;
        }
        $("#state").val("");

        automata = $.parseJSON(wasm.init_state(automata, value));//new_state

        var connections = $(".connection");
        $.each(connections, function(i, state){//each state new_state connected to
                let name = $(state).val();
                if (name != "") {
                    automata = $.parseJSON(wasm.init_state(automata, name));
                }
                var char = '';
                var emove = i >= automata["alphabet"].length;
                if(!emove){
                    char = automata["alphabet"].charAt(i);
                }
                automata = $.parseJSON(wasm.add_connection(automata, value, name, char, emove));
                $(state).val("");
            }
        );


        //add connections
        $.each(connections, function(i, state){//each state new_state connected to
            let name = $(state).val();
            if (name != "") {
                automata = $.parseJSON(wasm.init_state(automata, name));
            }
            $(state).val("");
            }
        );

        update_alphabet_display();
        update_states_display();
        update_start_end_states_display();
        
    });

    $("#set-alphabet-button").on("click",function(){
        var val = $("#alphabet").val();
        $("#alphabet").val("");
        automata = $.parseJSON(wasm.set_alphabet(automata, val));
        $("#initial-prompt").remove();
        update_alphabet_display();
        $(".init-hidden").css("visibility", "visible");

    });

    $("#transition-button").on("click", function(){
        transition();
    });

    $("#set-start-state-button").on("click", function(){
        automata = $.parseJSON(wasm.set_start_state(automata, $("#start-state").val()));
        if( automata["start_state"] != null){
            $("#start-state-section").remove();
        }
        else{
         $("#start-state").val("");
        }
        update_states_display();
        update_transition_button_display();
    });

    $("#set-end-states-button").on("click", function(){
        $.each($("#end-states").val().split(", "), function(i, val){
            automata = $.parseJSON(wasm.set_end_state(automata, val));
        });    
        if( automata["end_states"] != ""){
            $("#end-states-section").remove();
        }
        else{
            $("#end-states").val("");
        }
        update_states_display();
        update_transition_button_display();
    });

    $("#import-file").change(function(){       
        $.get("tests/" + $("#import-file").prop('files')[0]["name"], function(data){
            automata = JSON.parse(JSON.stringify(data));
            $("#initial-prompt").remove();
            update_states_display();
            update_transition_button_display()
            $(".init-hidden").css("visibility", "visible");
            $(".start-end-states").remove();
            update_alphabet_display();
        });

    });

    update_table_caption();
    update_transition_button_text();
});
