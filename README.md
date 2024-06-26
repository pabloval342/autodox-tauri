<h1 align="center">AUTODOX</h1>
<h3 align="center">SAVES YOUR TIME</h3>



<img align="left" src="https://i.ibb.co/xzJXxWK/logo2.png" style="width:100%">
AUTODOX

This app is Notion.so clone, roamresearch clone and obsidian clone. The main purpose of this app is not to just clone these note taking apps but to make an all in one **Open source note taking app** with automation features.

# learn about autodox here.
1. [documentations](https://docs.google.com/document/d/1lOJONK7QhtGGkzSGia37pD7fytTbpIoh5R8lXGuJ7Hw/edit)
1. [overview](https://lnkd.in/eUh_4JfV)
1. [Custom smart contracts](https://youtu.be/HqogGj0xKuE)
2. [privacy plan](https://lnkd.in/e6wC_-fW)
3. [the code](https://lnkd.in/eQyebWnH)
4. [project plan](https://lnkd.in/eUvpVi5P)
5. [drag and drop](https://lnkd.in/e9XDMjie)
6. [live collaboration](https://lnkd.in/ekTH46iu)


## features:
1. **Plugins** : plugins or extensions are customizations that you can add to your AUTODOX application. For example, you can add grammar correction plugin like grammarly, or a machine learning plugin that help you abbreviation your text.
2. **Components** : In notion you may noticed that you can import a table when you hit `/` then type table then hit enter. The table called a component. In AUTODOX you can create your own custom components. For example you can create flash cards. Also, you can use plugins to enhance your components. For example, you can use google translator plugin with flash card component so everytime you add a word you will get automatic translation.
3. **Services** : the is the core goal of AUTODOX where you can create a google translator plugin and create flashcard components then put them all in one workspace (page) and you can publish that page so other people can use it. In other word you don't need to create new plugin and component for every user, instead one user can create all of them and share it with others.
3. **search** : We will have 6 search functionaries.
    - search for words in file
    - search for files names
    - global search for words in any files in any directory you choose
    - regular expression search.
    - save your search results and reuse them again.

5. **spreadsheet** : similar to microsoft excel you will have a spreadsheet where you can store your data and implement formulas. Also, with plugins you can implement custom formulas like a google translator. Last but not least, you can use these spreadsheets as a backend for your services. In other word the components will act as a frontend that interact with this spreadsheet.
- Features:
    -  **Columns permisions**: You can decicde who can see, update each column or comment on a column
    -  **Views perimsion**: You can decicde who can see each view. Also, the views perimsion caled rows perimsion because you can add a fitler for a view to repvent spesfic users from sewing sepsfic rows. And this filter can't be removed by anyone so others can only add more filters but can't touch your filter.
    - **Custom smart contract**: you can write somthing like `if (approved = true) {transfer(100$).to(columns.assigned)}` this will automatically release the payment of a ser when you approve the delivery of an item.
    
7. **Ownership** : when you create a component, or a plugin or a service you will own it as an NFT. Hence, you can make money from it. There are three ways to make money with NFTs. One by selling it. Second, by getting percentage on every sell. Third, by requiring subscriptions fees like 7$ a month without selling the plugin or the service.

6. **Privacy** : You will have the option to upload your data on the block-chain internet computer in order to share it with other people.
 
## Benefits and vision
- First of all, i believe this new system will replace microsoft office and apple iwork.
- Users, will have safe place to store their data on the blockchain on IC.
- Users can do whatever they can imagine with all these customizations.
- there are more feature that I will work one like Real time data, so you can share your documents and update them in real time. or like page components so you can have an entire page as a spreadsheet.
- Success: I belive autodox will be secceful because the market is already huge, if I just take all docs apps like evernotes, microsoft words and notion then I put all of their featres in one place, of course I will get at least few users.
- Doablity: Because I am using rust and wasm everything became duable and easy, even the code editor. With rust I can exicute any program langauge seamlessly.
- I also ran few **experiment** before building this project in order to reach to the ultmate way.
    - [experiment 1](https://github.com/aliscie/autodox-tauri-react)
    - [other experiment](https://github.com/aliscie/autodox2)
    - [other experiment](https://github.com/aliscie/autdox)
    - [notion clone](https://github.com/aliscie/Notion.so-clone)
    - [second notion clone](https://github.com/aliscie/notion-clone-1)
    - [Other experment](https://github.com/AlenSci/autodox-1)
    - [experiment 2](https://github.com/aliscie/Learning-webstack-from-cloning-notion.so) 
    - [experiment 3](https://github.com/AlenSci/autodox1) [also see demo](https://www.youtube.com/watch?v=zXdL4B73Rkc) [and this](https://www.youtube.com/playlist?list=PLZ54FkZk9dwGrJSxLIm4-NLvHlhyQKL6T)
    - [experiment 4](https://github.com/AlenSci/autodox) 
    - [experiment 5](https://github.com/aliscie/autodox-rust)



## Development:
- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)


## Running the project locally
### configurations
1. check [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)
    especially the following prerequisites.
    1. $`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
        - or run $`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
    1. install xcode
       - to check that run $`xcode-select --install`
    1. make sure to have c++ 
### running the project
1. $`cd Desktop`
1. $`git clone https://github.com/aliscie/autodox-tauri`
1. $`cd autodox-tauri`
1. $`cargo tauri dev`
