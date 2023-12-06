<STDIN> =~ /seeds: (.*)/;
@seeds = split(' ', $1);
<STDIN> == '' || die;
sub Translate (\%\%$$$) {
    my ($endmapref, $diffmapref, $curkey, $id, $targetkey) = @_;
    #print "Translate $curkey id $id to $targetkey\n";
    my %endmaps = %$endmapref;
    my %diffmaps = %$diffmapref;
    until ($curkey eq $targetkey) {
        #print "$curkey id $id\n";
        for my $mapping (keys %endmaps) {
            if ($mapping =~ /^$curkey=>(.*)/) {
                my $destkey = $1;
                %endmap = %{ $endmaps{$mapping} };
                %diffmap = %{ $diffmaps{$mapping} };
                for my $range (keys %endmap) {
                    if ($id >= $range && $id < $endmap{$range}) {
                        $id += $diffmap{$range};
                        last;
                    }
                }
                $curkey = $destkey;
                last;
            }
        }
    }
    #print "$curkey id $id\n\n";
    return $id;
}
while (<STDIN> =~ /(.*)-to-(.*) map:/) {
    $src = $1;
    $dest = $2;
    $key = "$src=>$dest";
    $backkey = "$dest=>$src";
    while (<STDIN> =~ /(\d+) +(\d+) +(\d+)/) {
        $srcstart = int($2);
        $rangelen = int($3);
        $srcend = $srcstart + $rangelen;
        $deststart = int($1);
        $destend = $deststart + $rangelen;
        $endmaps{$key}{$srcstart} = $srcend;
        $diffmaps{$key}{$srcstart} = $deststart - $srcstart;
        $backendmaps{$backkey}{$deststart} = $destend;
        $backdiffmaps{$backkey}{$deststart} = $srcstart - $deststart;
        $extremum1 = Translate(%backendmaps, %backdiffmaps, $src, $srcstart, 'seed');
        $extremum2 = Translate(%backendmaps, %backdiffmaps, $src, $srcend, 'seed');
        $extremum{$extremum1} = 1;
        $extremum{$extremum2} = 1;
    }
}
@extremum = sort {$a <=> $b} keys %extremum;
#print "extremums: @extremum\n\n";
$targetkey = 'location';
($min1, $min2) = ('none', 'none');
for $i (@seeds) {
    $id = Translate(%endmaps, %diffmaps, 'seed', $i, 'location');
    if ($min1 eq 'none' || $min1 > $id) {
        $min1 = $id;
    }
}
while (1) {
    $seedrange = shift @seeds;
    #print "$seedrange\n";
    $seedrangeend = $seedrange + shift @seeds;
    for $i (($seedrange, @extremum)) {
        next if ($i < $seedrange || $i >= $seedrangeend);
        $id = Translate(%endmaps, %diffmaps, 'seed', $i, 'location');
        if ($min2 eq 'none' || $min2 > $id) {
            $min2 = $id;
        }
    }
    last if 0 == @seeds;
}
print "$min1\n$min2\n";
