%refcolors = ('red' => 12, 'green' => 13, 'blue' => 14);
$total1 = 0;
$total2 = 0;
LINE: while (<STDIN> =~ /^Game (\d+): (.*)$/) {
    $id = $1;
    %colors = ();
    foreach (split('; ', $2)) {
        foreach (split(', ', $_)) {
            ($n, $color) = split(' ', $_);
            $n = int($n);
            if (! exists $colors{$color} || $colors{$color} < $n) {
                $colors{$color} = $n;
            }
        }
    }
    $power = 1;
    foreach (values %colors) {
        $power *= $_;
    }
    $total2 += $power;
    foreach (keys %colors) {
        next LINE if ($colors{$_} > $refcolors{$_});
    }
    $total1 += $id;
}
print $total1, "\n", $total2, "\n";
