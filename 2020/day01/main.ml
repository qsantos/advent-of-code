let puzzle1 =
    let f = open_in "input" in
    let target = 2020 in
    let seen = Array.make target false in
    try
        while true; do
            let line = input_line f in
            let a = int_of_string line in
            let b = target - a in
            if 0 <= b && b < target && seen.(b) then begin
                Printf.printf "%d\n" (a * b);
                raise Exit
            end;
            seen.(a) <- true;
        done;
        close_in f
    with
    | Exit -> ()
    | End_of_file -> Printf.printf "Not found\n"
    | e ->
        close_in_noerr f;
        raise e
;;

let puzzle2 =
    let f = open_in "input" in
    let target = 2020 in
    let seen = Array.make target false in
    try
        while true; do
            let line = input_line f in
            let a = int_of_string line in
            for b = 0 to 2019; do
                let c = target - a - b in
                if seen.(b) && 0 <= c && c < target && seen.(c) then begin
                    Printf.printf "%d\n" (a * b * c);
                    raise Exit
                end;
            done;
            seen.(a) <- true;
        done;
        close_in f
    with
    | Exit -> ()
    | End_of_file -> Printf.printf "Not found\n"
    | e ->
        close_in_noerr f;
        raise e
;;

let main =
    puzzle1;
    puzzle2
;;

main
