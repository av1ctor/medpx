# Medpx
Your health data safe, under your control

![image](https://github.com/av1ctor/medpx/assets/35101441/41d2c91b-1dc8-45c6-9ff3-366d0c2010f7)

## Introduction
Medpx keeps your medical prescriptions on the blockchain using cryptography to ensure nobody can access them – unless you allow it.

- Cryptography – leveraging the VetKeys technology, present only on the Internet Computer, nobody can access your prescriptions, not even the host provider
- Shareable – You can share your prescriptions with other users, for example a Hospital, Drug store or another doctor, or create groups, allowing any users that are part of them to access your data
- Time-locked – When sharing your prescriptions, you can define a date limit to cut the access to your data
- Easy to find – You can create keys, like your e-mail, your phone number, your id card number, etc, so you can be found more easily by other users (no need to keep tracking of long and cumbersome wallet addresses)
- On-chain – Medpx is a decentralized web3 app, running 100% on-chain on the Internet Computer, giving you full control of your prescriptions
- Open-source – released under MIT license, anyone can verify the source code and contribute to the app

## Installation

### Prerequisites
- dfx 0.14+

```bash
$ git clone [https://github.com/av1ctor/medpx.git](https://github.com/av1ctor/medpx.git)
$ cd <project>
$ npm install
$ dfx start --background
$ dfx deploy
```

## Roadmap
- [x] All prescriptions should be encrypted using VetKeys
- [x] Patients should be able to share prescriptions with other users 
- [x] Patients should be able to share prescriptions with groups
- [x] A shared prescription can have a time limit
- [x] Users can create keys (like e-mail, phone number, id number, etc) to be found by other users more easily
- [x] A qr-code should be generated to allow patients to share their prescriptions with third-parties (if they are authorized)
- [ ] A prescription to be created should consume credits (our own ICRC-1 token)
- [ ] Doctors should be able to buy credits using ICP
- [ ] Multiple languages should be allowed
- [ ] Users must be verified by e-mail when signing up
- [ ] When creating keys, they must be verified (by SMS, by e-mail etc), to be sure the user owns them
- [ ] Controlled prescriptions (black-box drugs) should be used only-once, so the drug store must be allowed to mark them as used

## License
This project is licensed under the MIT license, see LICENSE file for details. 

## References
- [VetKey primer](https://internetcomputer.org/blog/features/vetkey-primer)
