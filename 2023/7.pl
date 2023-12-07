use feature "switch";

sub Card2Ord ($$) {
    my ($card, $subtask) = @_;
    if ($card eq 'T') { return 10; }
    if ($card eq 'J' && $subtask == 1) { return 11; }
    if ($card eq 'J' && $subtask == 2) { return 1; }
    if ($card eq 'Q') { return 12; }
    if ($card eq 'K') { return 13; }
    if ($card eq 'A') { return 14; }
    return int($card);
}

sub Hand2Ord ($$) {
    my ($card, $subtask) = @_;
    return map {Card2Ord($_, $subtask)} split('', $card);
}
sub Type (@) {
    my @hand = @_;
    my %hand;
    my $jokers;
    for my $card (@hand) {
        if ($card == 1) {
            $jokers++;
        } else {
            $hand{$card}++;
        }
    }
    my @ranks = values %hand;
    @ranks = sort {$b <=> $a} @ranks;
    if (@ranks[0] + $jokers == 5) { return 7; }
    if (@ranks[0] + $jokers == 4) { return 6; }
    if (@ranks[0] + $jokers == 3 && @ranks[1] == 2) { return 5; }
    if (@ranks[0] + $jokers == 3) { return 4; }
    if (@ranks[0] + $jokers == 2 && @ranks[1] == 2) { return 3; }
    if (@ranks[0] + $jokers == 2) { return 2; }
    return 1;
}
sub Sort ($$$%) {
    my ($a, $b, $subtask, %types) = @_;
    my $ret = $types{$a} <=> $types{$b};
    if ($ret == 0) {
        @a = Hand2Ord($a, $subtask);
        @b = Hand2Ord($b, $subtask);
        for ($i=0; $i < @a; $i++) {
            $ret = @a[$i] <=> @b[$i];
            last if ($ret != 0);
        }
    }
    return $ret;
}

while (<STDIN> =~ /^([2-9AKQJT]{5}) (\d+)$/) {
    $bids{$1} = int($2);
    $types1{$1} = Type(Hand2Ord($1, 1));
    $types2{$1} = Type(Hand2Ord($1, 2));
}
@cards = keys %bids;
@cards1 = sort {Sort($a, $b, 1, %types1)} @cards;
@cards2 = sort {Sort($a, $b, 2, %types2)} @cards;
$i = 1;
for (@cards1) {
    $total1 += $i * $bids{$_};
    $i++;
}
$i = 1;
for (@cards2) {
    $total2 += $i * $bids{$_};
    $i++;
}
print "$total1\n$total2\n";
