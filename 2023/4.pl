while (<STDIN> =~ /^Card +(?<id>\d+): ([\d ]+) \| ([\d ]+)$/) {
    $cards{$+{id}} += 1;
    %winning = map {int($_) => 1} split(' ', $2);
    @scored = map(int, split(' ', $3));
    @isect = grep {$winning{$_}} @scored;
    $total1 += 2 ** (@isect - 1) if @isect;
    for ($next = $+{id} + 1; $next <= $+{id} + @isect; $next++) {
        $cards{$next} += $cards{$+{id}};
    }
}
$total2 += $_ foreach (values %cards);
print "$total1\n$total2\n";
