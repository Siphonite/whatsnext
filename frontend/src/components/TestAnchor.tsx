import React, { useEffect } from "react";
import { useAnchorProgram } from "../hooks/useAnchorProgram";

const TestAnchor: React.FC = () => {
  const { program, provider, walletPubkey, connected } = useAnchorProgram();

  useEffect(() => {
    console.log("=========== TEST ANCHOR ===========");
    console.log("Wallet connected:", connected);
    console.log("Wallet pubkey:", walletPubkey?.toBase58() || "No wallet");

    if (provider) {
      console.log("Provider loaded:", provider);
    } else {
      console.error("Provider is NULL — wallet not connected or wallet adapter missing");
    }

    if (program) {
      console.log("Program loaded:", program);
      console.log("Program ID:", program.programId.toBase58());
    } else {
      console.error("Program is NULL — check IDL path or PROGRAM_ID in .env");
    }

    console.log("====================================");
  }, [program, provider, walletPubkey, connected]);

  return (
    <div className="text-white p-4">
      <h2 className="text-xl font-bold">Anchor Test Component</h2>
      <p>Open your browser console to see program + provider logs.</p>

      {!connected && <p className="text-red-400 mt-2">Please connect your wallet.</p>}

      {connected && program && (
        <p className="text-green-400 mt-2">Program Loaded Successfully!</p>
      )}
    </div>
  );
};

export default TestAnchor;
