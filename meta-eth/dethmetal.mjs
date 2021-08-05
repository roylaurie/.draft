
import Web3 from 'web3';
import fs from 'fs';
import readline from 'readline';
import util from 'util';

const web3 = new Web3(new Web3.providers.WebsocketProvider("ws://127.0.0.1:8546"));

export default class DethMetal {
    static storageAddr = '0x9538b538edb6c9884c14b1e19aa96a95ea2ce78d';
    static storageAbi = [{"inputs":[],"name":"retrieve","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"num","type":"uint256"}],"name":"store","outputs":[],"stateMutability":"nonpayable","type":"function"}];

    #account = null;
    #password = 'p4ssw0rd';

    constructor() {
        this.initAccount();
    }
        
    async getBalance() {
        return web3.eth.getBalance(this.#account.address);
    }

    createAccount() {
        const account = web3.eth.accounts.create();
        const encKeystore = web3.eth.accounts.encrypt(account.privateKey, this.#password);

        fs.writeFileSync('./account-keystore.json', JSON.stringify(encKeystore));
    }

    readAccount() {
        const encKeystore = JSON.parse( fs.readFileSync('./account-keystore.json') );
        const account = web3.eth.accounts.decrypt(encKeystore, this.#password);
        return account;
    }

    initAccount() {
        try {
            this.#account = this.readAccount();
        } catch (e) {
            this.createAccount();
            this.#account = this.readAccount();
        }

        web3.eth.defaultAccount = this.getDefaultAddress();
        web3.eth.accounts.wallet.add(this.#account);
    }

    getDefaultAddress() {
        return this.#account.address;
    }

    async getStoredNumber() {
        const cStorage = new web3.eth.Contract(DethMetal.storageAbi, DethMetal.storageAddr, { from: this.getDefaultAddress() });
        return cStorage.methods.retrieve().call();
    }

    async setStoredNumber(num) {
        const cStorage = new web3.eth.Contract(DethMetal.storageAbi, DethMetal.storageAddr, { from: this.#account.address, gas: 29000, gasPrice: 1 });
        return cStorage.methods.store(num).send();
    }
}

const dethMetal = new DethMetal();
console.log('address: ', dethMetal.getDefaultAddress());
console.log('balance: ', web3.utils.fromWei(await dethMetal.getBalance(), 'ether'));
console.log('stored number: ', await dethMetal.getStoredNumber());

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

rl.question('Enter a new number: ', number => {
    dethMetal.setStoredNumber(Number.parseInt(number))
    .then(() => {
        dethMetal.getStoredNumber()
        .then(num => {
            console.log('new stored number: ', num);
            process.exit(0);
        });
    });
});

