### Solana Bootcamp Lending Program

This is a simple lending protocol to give a general idea how these DeFi programs
work. The lending protocol will only have two assets, $SOL and $USDC. In this
lending protocol to be able to borrow assets we will make the accounts
over-collaterized. To do this you calculate the health factor of the account
which allows an account susceptible to liquidation. If the account falls below
the health facture, then it will be able to get liquidated.

The program will use the following main instructions:

- Deposit
- Withdraw
- Borrow
- Repay
- Liquidate
- Setup Accounts

The main accounts needed are:

- Bank Account for each asset that we want on the lending protocol.
- User Accounts
