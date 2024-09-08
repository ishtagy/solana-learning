import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StudentIntroProgram } from "../target/types/student_intro_program";
import { expect } from "chai";

describe("student-intro-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.StudentIntroProgram as Program<StudentIntroProgram>;

  const student = {
    name: "Dias",
    message: "some short message",
  };

  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId,
  );

  it("Student is added", async () => {
    
    const tx = await program.methods.createAccount(student.name, student.message).rpc();

    const account = await program.account.studentState.fetch(studentPda);
    
    expect(student.name === account.name);
    expect(student.message === account.message);
    expect(account.student === provider.wallet.publicKey);
  });

  it("Student message is updated", async () =>{
    const newMessage = "new message";

    await program.methods.updateAccount(student.name, newMessage).rpc();

    const account = await program.account.studentState.fetch(studentPda);

    expect(account.message === newMessage);
  })

  it("Delete student", async () =>{
    await program.methods.deleteAccount(student.name).rpc();
  })

});
