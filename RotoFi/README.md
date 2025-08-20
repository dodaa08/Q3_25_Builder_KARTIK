## 💸 RotoFi — Money Circles on Solana

RotoFi turns old‑school saving circles into on‑chain rituals. A small crew chips in every round, and the pot rolls to the next person in line. No spreadsheets. No group chats. Just code.

![Solana](https://img.shields.io/badge/Solana-Localnet-3ECF8E?logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blueviolet)

---

### The vibe

- ✨ **Trustless**: Rules are code. Payouts are automatic.
- ⏱️ **On time**: Each round has a deadline; no more chasing payments.
- 🤝 **Fair**: Join order sets the payout order. Everyone gets a turn.

---

### 60‑second tour

1) **Create** a circle with the amount, group size, and timing you want.
2) **Join** to reserve your spot in the payout line.
3) **Contribute** your share each round before the clock runs out.
4) **Payout** hits the next person in line at the scheduled time.
5) **Repeat** until everyone’s been paid.
6) **Close** the circle and wrap things up.

---

### Flow postcard

```bash
Organizer
  └─> create_circle
        ↓
Members arrive
  └─> join_cycle  (join order = payout order)
        ↓
Rounds roll
  ├─> submit_contribution (everyone pays in)
  └─> trigger_payout      (next in line gets the pot)
        ↓
All done
  └─> close_cycle
```

---

### Why you’ll like it

- 🎯 Simple: clear steps, clean flow
- 🔍 Transparent: everything happens on-chain
- ⚡ Fast & cheap: built on Solana with Anchor

---

### Try it in three commands

```bash
npm install
anchor build
anchor test
```

That’s it — the tests walk through the basic lifecycle.

---

### What is RotoFi, really?

- A tiny, opinionated Solana program for running money circles without manual coordination.
- A friendly template for experiments, demos, and community pilots.
- A way to turn “see you next month” into “see you on-chain.”
