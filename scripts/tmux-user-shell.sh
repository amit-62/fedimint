#!/usr/bin/env bash

source ./scripts/lib.sh

echo Waiting for fedimint start

# waits for rust to write to this pipe
STATUS=$(cat $FM_READY_FILE)
if [ "$STATUS" = "ERROR" ]
then
    echo "fedimint didn't start correctly"
    echo "See other panes for errors"
    exit 1
fi

scripts/pegin.sh 10000.0 | show_verbose_output

use_cln_gw

echo Funding gateway e-cash wallet ...
scripts/pegin.sh 20000.0 1 | show_verbose_output

echo Done!
echo
echo "This shell provides the following aliases:"
echo ""
echo "  fedimint-cli   - cli client to interact with the federation"
echo "  lightning-cli  - cli client for Core Lightning"
echo "  lncli          - cli client for LND"
echo "  bitcoin-cli    - cli client for bitcoind"
echo "  gateway-cln    - cli client for the CLN gateway"
echo "  gateway-lnd    - cli client for the LND gateway"
echo
echo "Use '--help' on each command for more information"
echo ""
echo "Important tmux key sequences:"
echo ""
echo "  ctrl+b <num>          - switching between panels (num: 1 or 2)"
echo "  ctrl+b :kill-session  - quit tmuxinator"
