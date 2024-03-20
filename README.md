### Summary

Let It Rust is a tool for red and blue teamers looking to identify all the domains part of Microsoft tenant and automatically check if they are availible to purchase using the Gandi API. While this no longer allows access (see articles in Further Reading) to the tenant there are still excellent reasons to investigate this when looking to attack or defend an Azure tenant.

This tool was inspired by AADInternals and Let It Go. The motivation behind this was to use the gandi API to get better accuracy, as using whois information is prone to false positives.

There is some caching performed during a run and onmicrosoft.com domains are skipped to speed up the process.

### Limitations/TODO

* Right now this tool failing to give answers for some domains like .id and .vn. It may say they are availible when they are not. This means while its more accurate than using whois, it doesnt yet solve the problem of need to do additional manual checking with the results.
* It's also a bit slow. It may be possible to speed this up by taking advantage of tokio async Rust, which been used in places in preperation for this. Gandi however implements rate limiting and we dont want to get banned so would need some careful testing. I'll get it done if I ever have to scan a lot of domains.
* The code used to sort out if it has a second level domain like .co.uk is a bit crude, it might get things wrong for some unusual cases.
* Error handling is poor, needs to print more detailed messages.
* I have only tested the application on Debian based Linux, but I expect it'll work on other flavours just fine.
* It is untested on MacOS, I expect it'll work if you compile it yourself.
* Similarally it'll also probably run on Windows, but the ANSI colours may not work and print weird characters control sequences instead. Again you'll need to compile this yourself.

### Running the Precompiled Binary

For Linux users no setup is required as everything statically compiled into a single binary, letitrust. Just mark it as executable (`chmod u+x ./letitrust`) and. You can download it from releases or compile it yourself.

### Self Compiling

If you wish to compile the application yourself, just install the rust compiler, cargo, and then compile it.

```git clone https://github.com/CR-DMcDonald/letitrust```

How to install the rust compiler
* https://www.rust-lang.org/tools/install

How to install cargo
* https://doc.rust-lang.org/cargo/getting-started/installation.html

Once these are installed you can run the following command which will compile the application under target/release/letitrust
```cargo build --release```

### Configuration

You will need a Gandi account and a Personal Access Token to use this tool. Specify your Gandi Personal Access Token in a `config.toml` file in the same directory as the binary. This justs need a single line, setting the settings gandi_pat to your token. Your token should be hex and 40 characters long. 

```gandi_pat = "YOUR_GANDI_PERSONAL_ACCESS_TOKEN"```

The token doesnt need any specific access, but as all tokens need at least one privilege I would recommend just providing the token "View Organisation" access.

### Usage

Example 1 - specify a domain on command line

```./letitrust -d example.com```

Example 2 - specify multiple domains on the command line comma seperated (no spaces)

```./letitrust -d example1.com,example2.com```

Example 3 - specify a text file containing domains to be tested, one per line

```./letitrust -f ./mylistofdomains```

### Sample Output

```./letitrust -d bbc.co.uk

██      ███████ ████████     ██ ████████     ██████  ██    ██ ███████ ████████ 
██      ██         ██        ██    ██        ██   ██ ██    ██ ██         ██    
██      █████      ██        ██    ██        ██████  ██    ██ ███████    ██    
██      ██         ██        ██    ██        ██   ██ ██    ██      ██    ██    
███████ ███████    ██        ██    ██        ██   ██  ██████  ███████    ██    

letitrust v0.1
Written by Darren McDonald, Cryptic Red

Checking bbc.co.uk... 
Found domain: s4c.cymru ... unavailable
Found domain: dyerspodcast.com ... unavailable
Found domain: riverpictures.co.uk ... unavailable
Found domain: bbcshop.com.au ... unavailable
Found domain: bbcpolska.com ... unavailable
Found domain: windmillroad.com ... unavailable
Found domain: tracklicensing.com ... unavailable
Found domain: bbcwwtest.com ... available
Found domain: TE.extra.bbc.com ... converted to bbc.com ... unavailable
Found domain: beeb.net ... unavailable
Found domain: twoentertain.co.uk ... available
Found domain: wkbbc.com.cn ... available
Found domain: bbc.com ... unavailable
Found domain: topgear.com ... unavailable
Found domain: s4c.wales ... unavailable
Found domain: bbcnordic.com ... unavailable
Found domain: motdmag.co.uk ... available
Found domain: onebbc.onmicrosoft.com ... skipped
Found domain: onebbc.mail.onmicrosoft.com ... skipped
Found domain: main.bbc.co.uk ... converted to bbc.co.uk ... unavailable
Found domain: bbc.co.uk ... unavailable
Found domain: BBCtopgearmagazine.com ... unavailable
Found domain: 7e5010.msamer01.cloud-voice.io ... converted to cloud-voice.io ... unavailable
Found domain: bbccanada.ca ... unavailable
Found domain: bbclifestyle.co.za ... unavailable
Found domain: bbcmotiongallery.com ... unavailable
Found domain: bbcsouthafrica.com ... unavailable
Found domain: 2entertain.co.uk ... unavailable
Found domain: bbcentertainment.com ... unavailable
Found domain: 7e5010.msapac01.cloud-voice.io ... converted to cloud-voice.io ... unavailable
Found domain: s4c.co.uk ... unavailable
Found domain: demonmusicgroup.co.uk ... unavailable
Found domain: bbcfootage.com ... unavailable
Found domain: bestofbritainstore.com ... available
Found domain: bbcgermany.de ... unavailable
Found domain: bbcantiquesroadshowmag.com ... available
Found domain: bbcworldwide.com ... unavailable
Found domain: 7e5010.msemea01.cloud-voice.io ... converted to cloud-voice.io ... unavailable
Found domain: crimsonrecords.com ... unavailable
Found domain: bbcaudiobooks.co.uk ... unavailable
Found domain: bbcmagazinesadvertising.com ... unavailable
Found domain: doctorwhoexperience.com ... unavailable
Found domain: bbcaudiobooks.com ... unavailable
Found domain: bbclaproduction.com ... available
Found domain: bbc-asia.com ... unavailable
Found domain: bbcbrit.com ... unavailable
```

### Further Reading

If you have no idea what any of this means but want to know more, I can recommend the following URLs to get you upto speed.

https://sra.io/blog/letitgo-a-case-study-in-expired-domains-and-azure-ad/
https://github.com/SecurityRiskAdvisors/letItGo
https://aadinternals.com/aadinternals/#invoke-aadintreconasoutsider
https://api.gandi.net/docs/authentication/
https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html
