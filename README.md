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

```
./letitrust -d bbc.co.uk

██      ███████ ████████     ██ ████████     ██████  ██    ██ ███████ ████████ 
██      ██         ██        ██    ██        ██   ██ ██    ██ ██         ██    
██      █████      ██        ██    ██        ██████  ██    ██ ███████    ██    
██      ██         ██        ██    ██        ██   ██ ██    ██      ██    ██    
███████ ███████    ██        ██    ██        ██   ██  ██████  ███████    ██    

letitrust v0.1
Written by Darren McDonald, Cryptic Red

Checking bbc.co.uk... unavailable
  bbc.co.uk has a softfail spf record
Found spf include: spf.sis.bbc.co.uk ... converted to bbc.co.uk ... unavailable
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  bbc.co.uk has a DMARC record with p=none
Found Azure Tenant domain: bbcgermany.de ... unavailable
  bbcgermany.de has a DMARC record with p=none
Found Azure Tenant domain: main.bbc.co.uk ... converted to bbc.co.uk ... unavailable
  bbc.co.uk has a softfail spf record
Found spf include: spf.sis.bbc.co.uk ... converted to bbc.co.uk ... unavailable
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  bbc.co.uk has a DMARC record with p=none
Found Azure Tenant domain: onebbc.mail.onmicrosoft.com ... skipped
Found Azure Tenant domain: bbcaudiobooks.co.uk ... unavailable
  bbcaudiobooks.co.uk does not have an spf record
  bbcaudiobooks.co.uk does not have a DMARC record
Found Azure Tenant domain: bbcsouthafrica.com ... unavailable
  bbcsouthafrica.com does not have a DMARC record
Found Azure Tenant domain: bbclaproduction.com ... available
  bbclaproduction.com does not have an spf record
  bbclaproduction.com does not have a DMARC record
Found Azure Tenant domain: s4c.cymru ... unavailable
  s4c.cymru has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
Found spf include: ciphr247.com ... unavailable
  s4c.cymru has a DMARC record with p=none
Found Azure Tenant domain: twoentertain.co.uk ... available
  twoentertain.co.uk does not have an spf record
  twoentertain.co.uk does not have a DMARC record
Found Azure Tenant domain: demonmusicgroup.co.uk ... unavailable
  demonmusicgroup.co.uk has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  demonmusicgroup.co.uk has a DMARC record with p=none
Found Azure Tenant domain: bbcpolska.com ... unavailable
  bbcpolska.com has a DMARC record with p=none
Found Azure Tenant domain: 7e5010.msapac01.cloud-voice.io ... converted to cloud-voice.io ... unavailable
Found spf include: spf.mailjet.com ... converted to mailjet.com ... unavailable
Found Azure Tenant domain: bbcbrit.com ... unavailable
  bbcbrit.com does not have a DMARC record
Found Azure Tenant domain: motdmag.co.uk ... available
  motdmag.co.uk does not have an spf record
  motdmag.co.uk does not have a DMARC record
Found Azure Tenant domain: 7e5010.msemea01.cloud-voice.io ... converted to cloud-voice.io ... unavailable
Found spf include: spf.mailjet.com ... converted to mailjet.com ... unavailable
Found Azure Tenant domain: s4c.wales ... unavailable
  s4c.wales has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  s4c.wales has a DMARC record with p=none
Found Azure Tenant domain: bbcwwtest.com ... available
  bbcwwtest.com does not have an spf record
  bbcwwtest.com does not have a DMARC record
Found Azure Tenant domain: bbc-asia.com ... unavailable
  bbc-asia.com has a DMARC record with p=none
Found Azure Tenant domain: 7e5010.msamer01.cloud-voice.io ... converted to cloud-voice.io ... unavailable
Found spf include: spf.mailjet.com ... converted to mailjet.com ... unavailable
Found Azure Tenant domain: s4c.co.uk ... unavailable
  s4c.co.uk has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  s4c.co.uk has a DMARC record with p=none
Found Azure Tenant domain: bbclifestyle.co.za ... unavailable
  bbclifestyle.co.za does not have an spf record
  bbclifestyle.co.za does not have a DMARC record
Found Azure Tenant domain: crimsonrecords.com ... unavailable
  crimsonrecords.com has a DMARC record with p=none
Found Azure Tenant domain: bbcfootage.com ... unavailable
  bbcfootage.com has a softfail spf record
Found spf include: spf.sender.xserver.jp ... converted to xserver.jp ... unavailable
  bbcfootage.com does not have a DMARC record
Found Azure Tenant domain: beeb.net ... unavailable
  beeb.net does not have an spf record
  beeb.net does not have a DMARC record
Found Azure Tenant domain: bbcantiquesroadshowmag.com ... available
  bbcantiquesroadshowmag.com does not have an spf record
  bbcantiquesroadshowmag.com does not have a DMARC record
Found Azure Tenant domain: bbc.com ... unavailable
  bbc.com has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  bbc.com has a DMARC record with p=none
Found Azure Tenant domain: bbc.co.uk ... unavailable
  bbc.co.uk has a softfail spf record
Found spf include: spf.sis.bbc.co.uk ... converted to bbc.co.uk ... unavailable
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  bbc.co.uk has a DMARC record with p=none
Found Azure Tenant domain: bbcworldwide.com ... unavailable
Found Azure Tenant domain: bbccanada.ca ... unavailable
  bbccanada.ca does not have an spf record
  bbccanada.ca does not have a DMARC record
Found Azure Tenant domain: 2entertain.co.uk ... unavailable
  2entertain.co.uk has a DMARC record with p=none
Found Azure Tenant domain: tracklicensing.com ... unavailable
  tracklicensing.com has a DMARC record with p=none
Found Azure Tenant domain: bbcmotiongallery.com ... unavailable
  bbcmotiongallery.com has a DMARC record with p=none
Found Azure Tenant domain: bbcnordic.com ... unavailable
  bbcnordic.com does not have a DMARC record
Found Azure Tenant domain: dyerspodcast.com ... unavailable
  dyerspodcast.com has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
Found Azure Tenant domain: BBCtopgearmagazine.com ... unavailable
  BBCtopgearmagazine.com has a softfail spf record
Found spf include: spf.exclaimer.net ... converted to exclaimer.net ... unavailable
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
Found spf include: eu._netblocks.mimecast.com ... converted to mimecast.com ... unavailable
  BBCtopgearmagazine.com has a DMARC record with p=none
Found Azure Tenant domain: doctorwhoexperience.com ... unavailable
  doctorwhoexperience.com does not have an spf record
  doctorwhoexperience.com does not have a DMARC record
Found Azure Tenant domain: bbcaudiobooks.com ... unavailable
  bbcaudiobooks.com does not have an spf record
  bbcaudiobooks.com has a DMARC record with p=none
Found Azure Tenant domain: topgear.com ... unavailable
Found spf include: msgfocus.com ... unavailable
  topgear.com has a DMARC record with p=none
Found Azure Tenant domain: riverpictures.co.uk ... unavailable
  riverpictures.co.uk has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
Found Azure Tenant domain: bbcmagazinesadvertising.com ... unavailable
  bbcmagazinesadvertising.com does not have a DMARC record
Found Azure Tenant domain: TE.extra.bbc.com ... converted to bbc.com ... unavailable
  bbc.com has a softfail spf record
Found spf include: spf.messagelabs.com ... converted to messagelabs.com ... unavailable
  bbc.com has a DMARC record with p=none
Found Azure Tenant domain: bbcshop.com.au ... unavailable
  bbcshop.com.au has a softfail spf record
  bbcshop.com.au does not have a DMARC record
Found Azure Tenant domain: wkbbc.com.cn ... available
  wkbbc.com.cn does not have an spf record
  wkbbc.com.cn does not have a DMARC record
Found Azure Tenant domain: bbcentertainment.com ... unavailable
  bbcentertainment.com does not have a DMARC record
Found Azure Tenant domain: bestofbritainstore.com ... available
  bestofbritainstore.com does not have an spf record
  bestofbritainstore.com does not have a DMARC record
Found Azure Tenant domain: windmillroad.com ... unavailable
Found Azure Tenant domain: onebbc.onmicrosoft.com ... skipped
```

### Further Reading

If you have no idea what any of this means but want to know more, I can recommend the following URLs to get you upto speed.

https://sra.io/blog/letitgo-a-case-study-in-expired-domains-and-azure-ad/
https://github.com/SecurityRiskAdvisors/letItGo
https://aadinternals.com/aadinternals/#invoke-aadintreconasoutsider
https://api.gandi.net/docs/authentication/
https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html
