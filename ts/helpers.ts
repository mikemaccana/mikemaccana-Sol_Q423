const confirm = async (signature: string): Promise<string> => {
  const block = await provider.connection.getLatestBlockhash();
  await provider.connection.confirmTransaction({
    signature,
    ...block,
  });
  return signature;
};

const log = async (signature: string): Promise<string> => {
  console.log(
    `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`
  );
  return signature;
};
