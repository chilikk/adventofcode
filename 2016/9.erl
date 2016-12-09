-module('9').

-export([main/1]).

main(_) ->
    Len = process(fun() -> lazy_input() end, normal, 0),
    io:format("~B~n", [Len]).

process([$\n|Rest], S, Acc) ->
    process(Rest, S, Acc);
process([$ |Rest], S, Acc) ->
    process(Rest, S, Acc);
process(Input, {collect, 0, N, Coll}, Acc) ->
    Expand = process(lists:reverse(Coll)++eof, normal, 0),
    process(Input, {collect_expanded, 0, N, Expand}, Acc);
process(Input, {collect_expanded, 0, N, Coll}, Acc) ->
    process(Input, normal, N*Coll + Acc);
process([C|Rest], {collect, Count, N, Coll}, Acc) ->
    process(Rest, {collect, Count-1, N, [C|Coll]}, Acc);
process([$(|Rest], normal, Acc) ->
    process(Rest, {instruction, []}, Acc);
process([$)|Rest], {instruction, I0}, Acc) ->
    I = lists:reverse(I0),
    [Chars, N] = lists:map(fun erlang:binary_to_integer/1, re:split(I, "x")),
    process(Rest, {collect, Chars, N, []}, Acc);
process([C|Rest], {instruction, I}, Acc) ->
    process(Rest, {instruction, [C|I]}, Acc);
process([_|Rest], normal, Acc) ->
    process(Rest, normal, 1+Acc);
process(eof, _, Acc) ->
    Acc;
process(Fun, S, Acc) ->
    process(Fun(), S, Acc).

lazy_input() ->
    case io:get_chars([], 10) of
        L when is_list(L) -> L ++ fun() -> lazy_input() end;
        Eof -> Eof
    end.
