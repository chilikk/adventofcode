@lines = <STDIN>;
$strlen = length(@lines[0])-1;
$input = join('', @lines);
while ($input =~ /[^\n .0-9]/g) {
    $symbolpos{int(@-[0])} = 0;
}
while ($input =~ /[*]/g) {
    $gearpos{int(@-[0])} = 0;
    $gearratio{int(@-[0])} = 1;
}
while ($input =~ /([0-9]+)/g) {
    ($n, $b, $e) = (int($1), int($-[0]), int($+[0])-1);
    @adj = ($b-1, $e+1, $b-$strlen-2, $b+$strlen, $e-$strlen, $e+$strlen+2);
    for ($i = $b; $i <= $e; $i++) {
        push @adj, ($i-$strlen-1, $i+$strlen+1);
    }
    foreach (@adj) {
        if (exists($symbolpos{$_})) {
            $total1 += $n;
            break;
        }
    }
    foreach (@adj) {
        if (exists($gearpos{$_})) {
            $gearpos{$_}++;
            $gearratio{$_} *= $n;
        }
    }
}
foreach (keys %gearpos) {
    $total2 += $gearratio{$_} if ($gearpos{$_} == 2);
}
print "$total1\n$total2\n";
