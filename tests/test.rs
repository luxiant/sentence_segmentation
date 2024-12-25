use sentence_segmentation::processor;

#[test]
fn test_sentence_segmentation() {
    println!("\nTesting sentence segmentation for afrikaans.");
    let sentences = processor::afrikaans("Hoe gaan dit? 1. Die projek is belangrik, maar 2. ons moet hierdie webwerf besoek: https://voorbeeld.com. \t \"Ek dink nie... dit maak sin nie!\" sê hy. \r Byvoorbeeld: Hierdie *** is nie regtig nodig nie.  ... Sal jy môre kom? Ja, miskien (of nie).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for albanian.");
    let sentences = processor::albanian("Si je? 1. Projekti duhet përfunduar. 2. Shiko këtë faqe: www.shembull.com!  \n \"Nuk është... e lehtë,\" tha ai. \t Për shembull, ky është një **problem i pazakontë**.  ... A do të vish nesër? Po, ndoshta (ose jo).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for amharic.");
    let sentences = processor::amharic("እንዴት? 1. እንቅስቃሴውን መቆጣጠር አስፈላጊ ነው። 2. አድራሻ: www.example.com እና እንደ ግምት: \"ስለ ተሳስተህ እንዲሆን!\" (እባክህ።) \n \t ለምሳሌ፣ የልዩ ምርጫዎችን አስቀምጦ አስታውሳለሁ። ... እንደዚህ ምንም አይደለም።");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for arabic.");
    let sentences = processor::arabic("كيف حالك؟ 1. يجب إكمال المشروع. 2. قم بزيارة هذا الموقع: https://aaa.com.  \n \"هذا ليس... سهلًا\"، قال. \t على سبيل المثال: هذا **مشكلة غير عادية**.  ... هل ستأتي غدًا؟ نعم، ربما (أو لا).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for armenian.");
    let sentences = processor::armenian("Ինչպե՞ս ես։ 1. Նախագիծը պետք է ավարտվի։ 2. Այցելիր այս կայքը՝ www.orinak.am։  \n «Սա այնքան էլ... հեշտ չէ», - ասաց նա։ \t Օրինակ՝ սա ***անսովոր խնդիր է***։  ... Վաղը կգա՞ս: Այո, միգուցե (կամ ոչ)։");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for assamese.");
    let sentences = processor::assamese("কেনেকৈ আছা? 1. প্ৰকল্পটো সম্পূৰ্ণ হ'ব লাগিব। 2. এই ৱেবছাইটটোত যোৱা চাওক: www.example.com। \n «এইটো একেবাৰে... সহজ নহয়», - সেয়া সি কৈছিল। \t উদাহৰণস্বৰূপে, এইটো ***অস্বাভাবিক সমস্যা***। ... কাইলৈ আহিবা? হয়তো, সম্ভৱত (অথবা নহয়)।");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for azerbaijani.");
    let sentences = processor::azerbaijani("Necəsən? 1. Layihəni tamamlamaq vacibdir. 2. Bu sayta baxın: https://nümunə.az.  \n \"Bu... asan deyil,\" dedi. \t Məsələn: Bu ***qeyri-adi bir problemdir***.  ... Sabah gələcəksən? Bəli, bəlkə də (ya da yox).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for balinese.");
    let sentences = processor::balinese("Kamusta? 1. Proyèkna kudu rampung. 2. Mangga buka situs iki: www.contoh.com. \n \"Ipun boten saget... gampang,\" wangsulipun piyambak. \t Conto, punika ***masalah ingkang ora umum***. ... Esuk sampeyan rawuh? Mungkin, saged ugi boten.");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for basque.");
    let sentences = processor::basque("Zelan zaude? 1. Proiektua amaitzea garrantzitsua da. 2. Ikusi webgune hau: www.adibidea.eus.  \n \"Hau... ez da erraza,\" esan zuen. \t Adibidez: Hau ***ohiz kanpoko arazo bat*** da.  ... Bihar etorriko zara? Bai, agian (edo ez).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for belarusian.");
    let sentences = processor::belarusian("Як ты? 1. Праект трэба завяршыць. 2. Наведай гэты сайт: www.aaaa.by.  \n \"Гэта... няпроста,\" — сказаў ён. \t Напрыклад: Гэта ***незвычайная праблема***.  ... Ты прыйдзеш заўтра? Так, магчыма (ці не).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for bengali.");
    let sentences = processor::bengali("কেমন আছো? 1. প্রকল্পটি সম্পন্ন করা জরুরি। 2. এই ওয়েবসাইটটি দেখো: www.abc.com.  \n \"এটা... সহজ নয়,\" সে বলল। \t উদাহরণস্বরূপ: এটি ***অস্বাভাবিক সমস্যা***।  ... তুমি কি কাল আসবে? হ্যাঁ, হয়তো (বা না)।");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for bosnian.");
    let sentences = processor::bosnian("Kako si? 1. Projekat treba biti završen. 2. Posjeti ovu stranicu: www.primjer.ba.  \n \"Ovo... nije lako,\" rekao je. \t Na primjer: Ovo je ***neobičan problem***.  ... Hoćeš li doći sutra? Da, možda (ili ne).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for bulgarian.");
    let sentences = processor::bulgarian("Как си? 1. Проектът трябва да бъде завършен. 2. Посети този сайт: www.example.bg.  \n \"Това... не е лесно,\" каза той. \t Например: Това е ***необичайна проблема***.  ... Ще дойдеш ли утре? Да, може би (или не).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for burmese.");
    let sentences = processor::burmese("နေကြတယ်လား? 1. ပရောဂျက်ကိုပြီးစီးရမည်။ 2. ဤဝက်ဘ်ဆိုဒ်ကိုကြည့်ပါ: abac.com.  \n \"ဒါ...လွယ်ကူခြင်းမဟုတ်ပါ,\" သူပြောသည်။ \t ဥပမာအားဖြင့်: ဤသည် ***ထူးခြားသောပြဿနာ*** ဖြစ်သည်။  ... မနက်ဖြန်လာမလား? ဟုတ်တယ်၊ အခြားအခြေအနေများမှာ (သို့မဟုတ် မဟုတ်)။");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for catalan.");
    let sentences = processor::catalan("Com estàs? 1. El projecte ha de ser completat. 2. Visita aquest lloc web: www.exemple.cat.  \n \"Això... no és fàcil,\" va dir ell. \t Per exemple: Aquest és un ***problema inusual***.  ... Vindràs demà? Sí, potser (o no).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for cebuano.");
    let sentences = processor::cebuano("Kumusta ka? 1. Kinahanglan matapos ang proyekto. 2. Bisitahi ang website: www.example.com. \n \"Dili kini... sayon,\" ingon niya. \t Pananglitan, kini usa ka ***katingad-an nga problema***. ... Moanhi ba ka ugma? Siguro, tingali (o dili).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for chechen.");
    let sentences = processor::chechen("Сий да? 1. Проект хьо заверш. 2. Хьо хӏерар сайт: www.example.com. \n \"Хьо оьрш езар... кхетар,\" - хьа кхети. \t Мисал, хьо ***сахьийла проблемах***. ... Дийла хӏала? Можна, хьо йолу (или хьо ду).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for chinese.");
    let sentences = processor::chinese("你好吗？1. 项目需要完成。2. 请访问这个网站： www.aaa.cn。  \n \"这个...不容易，\"他说。 \t 例如：这是一个 ***不寻常的问题***。  ... 明天你会来吗？ 是的，也许（或者不是）。");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for creole.");
    let sentences = processor::creole("Kijan ou ye? 1. Pwojè a dwe fini. 2. Ale sou sit entènèt sa a: www.example.com. \n \"Sa pa... fasil,\" li di. \t Pa egzanp, sa a se yon ***pwoblèm ki ra***. ... Ou pral vini demen? Petèt, swa wi (oswa non).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for croatian.");
    let sentences = processor::croatian("Kako si? 1. Projekt treba biti završen. 2. Posjetite ovu stranicu: www.prikaz.hr.  \n \"Ovo... nije lako,\" rekao je. \t Na primjer: Ovo je ***neobičan problem***.  ... Hoćeš li doći sutra? Da, možda (ili ne).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for czech.");
    let sentences = processor::czech("Jak se máš? 1. Projekt musí být dokončen. 2. Navštivte tuto stránku: www.pralad.cz.  \n \"To... není snadné,\" řekl. \t Například: Toto je ***neobvyklý problém***.  ... Přijdeš zítra? Ano, možná (nebo ne).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for danish.");
    let sentences = processor::danish("Hvordan har du det? 1. Projektet skal afsluttes. 2. Besøg denne hjemmeside: www.eksempel.dk.  \n \"Dette... er ikke nemt,\" sagde han. \t For eksempel: Dette er en ***usædvanlig problem***.  ... Kommer du i morgen? Ja, måske (eller ikke).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for dinka.");
    let sentences = processor::dinka("Nɛɛɣɔɔ? 1. Kɔmɔlɔk nɛ ɛɔ̈t. 2. Jɔk ci website: www.example.com. \n \"Mɛl kɔɔr... ra,\" ɣɔr. \t Lɔm, kɔɔr ***ɛtɔ̈ɔt kɔɔr***. ... Kɔɔkɔɔ ɛtɛɛ? Nɛɛ, bɛɛ nɛ (ɔɔ bɛɛ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for dutch.");
    let sentences = processor::dutch("Hoe gaat het? 1. Het project moet worden voltooid. 2. Bezoek deze website: www.voorbeeld.nl.  \n \"Dit... is niet gemakkelijk,\" zei hij. \t Bijvoorbeeld: Dit is een ***ongebruikelijk probleem***.  ... Kom je morgen? Ja, misschien (of niet).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for english.");
    let sentences = processor::english("How are you? 1. The project needs to be completed. 2. Visit this website: www.example.com.  \n \"This... isn't easy,\" he said. \t For example: This is a ***very unusual problem***.  ... Will you come tomorrow? Yes, maybe (or not).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for esperanto.");
    let sentences = processor::esperanto("Kiel vi fartas? 1. La projekto devas esti finita. 2. Vizitu ĉi tiun retejon: www.ekzemplo.org.  \n \"Tio... ne estas facila,\" li diris. \t Ekzemple: Ĉi tio estas ***malkutima problemo***.  ... Ĉu vi venos morgaŭ? Jes, eble (aŭ ne).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for estonian.");
    let sentences = processor::estonian("Kuidas sul läheb? 1. Projekt tuleb lõpetada. 2. Külastage seda veebisaiti: www.naide.ee.  \n \"See... ei ole lihtne,\" ütles ta. \t Näiteks: See on ***ebatavaline probleem***.  ... Kas sa tuled homme? Jah, võib-olla (või mitte).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for finnish.");
    let sentences = processor::finnish("Miten menee? 1. Projekti täytyy saattaa päätökseen. 2. Käy tässä verkkosivustossa: www.esimerkki.fi.  \n \"Tämä... ei ole helppoa,\" hän sanoi. \t Esimerkiksi: Tämä on ***epätavallinen ongelma***.  ... Tuletko huomenna? Kyllä, ehkä (tai ei).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for french.");
    let sentences = processor::french("Comment ça va ? 1. Le projet doit être terminé. 2. Visitez ce site web : www.exemple.fr.  \n \"Cela... n'est pas facile,\" a-t-il dit. \t Par exemple : C'est un ***problème inhabituel***.  ... Viens-tu demain ? Oui, peut-être (ou pas).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for galician.");
    let sentences = processor::galician("Como estás? 1. O proxecto debe rematar. 2. Visita este sitio web: www.example.com. \n \"Isto non é... doado,\" dixo el. \t Exemplificando, isto é un ***problema inusual***. ... Virás mañá? Quizais, pode que si (ou non).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for ganda.");
    let sentences = processor::ganda("Oli otem? 1. Eyin omukubuga kwekikozese. 2. Funa oluwebusaiti: www.example.ug.  \n \"Ono... tekikyo kya,\" yagamba. \t Ekyokulabirako: Kino kye ***kizibu ekitali kimu***.  ... Ojja ola? Yee, ennaku (oba nedda).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for georgian.");
    let sentences = processor::georgian("როგორ ხარ? 1. პროექტი უნდა დასრულდეს. 2. ეწვიეთ ამ ვებსაიტს: www.example.ge.  \n \"ეს... არ არის ადვილი,\" თქვა მან. \t მაგალითად: ეს არის ***უპირველეს ყოვლისა პრობლემა***.  ... იქნები ხვალ? კი, ალბათ (ან არა).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for german.");
    let sentences = processor::german("Wie geht's dir? 1. Das Projekt muss abgeschlossen werden. 2. Besuche diese Website: www.beispiel.de.  \n \"Das... ist nicht einfach,\" sagte er. \t Zum Beispiel: Dies ist ein ***sehr ungewöhnliches Problem***.  ... Kommst du morgen? Ja, vielleicht (oder nicht).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for greek.");
    let sentences = processor::greek("Πώς είσαι; 1. Το έργο πρέπει να ολοκληρωθεί. 2. Επισκεφτείτε αυτή την ιστοσελίδα: www.paradeigma.gr.  \n \"Αυτό... δεν είναι εύκολο,\" είπε. \t Για παράδειγμα: Αυτό είναι ένα ***πολύ ασυνήθιστο πρόβλημα***.  ... Θα έρθεις αύριο; Ναι, ίσως (ή όχι).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for gujarati.");
    let sentences = processor::gujarati("તમારો હાલ કયા છે? 1. પ્રોજેક્ટ પૂર્ણ કરવો જોઈએ. 2. આ વેબસાઇટ પર જાઓ: www.udaharan.in.  \n \"આ... સરળ નથી,\" તે કહેતા હતા. \t ઉદાહરણ તરીકે: આ એક ***વિશિષ્ટ સમસ્યા*** છે.  ... તમે કાલે આવશે? હા, કદાચ (કિંમત નહિ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for hausa.");
    let sentences = processor::hausa("Yaya kake? 1. Aikin yana bukatar a kammala. 2. Ziyarci wannan shafin yanar gizo: www.misali.com.  \n \"Wannan... ba shi da sauƙi,\" in ji shi. \t Misali: Wannan matsala ce ***mai wuya sosai***.  ... Za ka zo gobe? Eh, wata kila (ko a'a).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for hebrew.");
    let sentences = processor::hebrew("מה שלומך? 1. עליך לסיים את הפרויקט. 2. בקר באתר זה: www.example.co.il.  \n \"זה... לא פשוט,\" אמר הוא. \t לדוגמה: זו בעיה ***לא רגילה***.  ... תבוא מחר? כן, אולי (או לא).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for hindi.");
    let sentences = processor::hindi("कैसा है? 1. परियोजना को पूरा किया जाना चाहिए। 2. इस वेबसाइट पर जाएं: www.example.in.  \n \"यह... आसान नहीं है,\" उसने कहा। \t उदाहरण के लिए: यह एक ***असामान्य समस्या*** है।  ... क्या तुम कल आओगे? हां, शायद (या नहीं).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for hungarian.");
    let sentences = processor::hungarian("Hogy vagy? 1. A projektet be kell fejezni. 2. Látogass el erre az oldalra: www.pelda.hu.  \n \"Ez... nem könnyű,\" mondta. \t Például: Ez egy ***rendkívül szokatlan probléma***.  ... Jössz holnap? Igen, talán (vagy nem).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for icelandic.");
    let sentences = processor::icelandic("Hvernig hefur þú það? 1. Verkefnið þarf að klárast. 2. Farðu á þessa vefsíðu: www.dmi.is.  \n \"Þetta... er ekki auðvelt,\" sagði hann. \t Til dæmis: Þetta er ***mjög óvenjulegt vandamál***.  ... Komstu á morgun? Já, kannski (eða ekki).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for igbo.");
    let sentences = processor::igbo("Kedu ka? 1. A ga-emecha oru a. 2. Gaa na saịtị a: www.nkeoma.com.  \n \"Nke a... adịghị mfe,\" ka o kwuru. \t Dịka ọmụmaatụ: Nke a bụ ***ajụjụ pụrụ iche***.  ... Ị ga-abịa echi? Ee, ma eleghị anya (ma ọ bụ na-adịghị).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for ido.");
    let sentences = processor::ido("Iē būn? 1. Cōlōhēn nān bāyōn. 2. Māyāṅ to śǒtē: www.example.com. \n \"Sīrēn tō... rōn,\" hēn bāyōn. \t Cāmlōhēn, hēn ***bīnēgōn rēbēlō***. ... Dāngāwōn ōlān? Kīzā, hēn zē (sā hēn dē).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for indonesian.");
    let sentences = processor::indonesian("Apa kabar? 1. Proyek ini harus diselesaikan. 2. Kunjungi situs web ini: www.contoh.co.id.  \n \"Ini... tidak mudah,\" katanya. \t Sebagai contoh: Ini adalah masalah ***yang sangat tidak biasa***.  ... Apakah kamu datang besok? Ya, mungkin (atau tidak).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for interlingua.");
    let sentences = processor::interlingua("Como sta? 1. Le projecto debe terminar. 2. Visita iste sito web: www.example.com. \n \"Isto non es... facille,\" ille diceva. \t Per exemplo, isto es un ***problema inusual***. ... Venira tu morgen? Talvez, forsan si (o no).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for irish.");
    let sentences = processor::irish("Conas atá tú? 1. Caithfidh an tionscadal a bheith críochnaithe. 2. Tabhair cuairt ar an suíomh gréasáin seo: www.sampla.ie.  \n \"Ní héasca é seo...,\" ar sé. \t Mar shampla: Is fadhb ***neamhghnách*** í seo.  ... An mbeidh tú ann amárach? Sea, b’fhéidir (nó ní hea).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for italian.");
    let sentences = processor::italian("Come stai? 1. Il progetto deve essere completato. 2. Visita questo sito web: www.esempio.it.  \n \"Questo... non è facile,\" ha detto. \t Ad esempio: Questo è un problema ***molto insolito***.  ... Verrà domani? Sì, forse (o no).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for japanese.");
    let sentences = processor::japanese("お元気ですか？ 1. プロジェクトは完了する必要があります。 2. このウェブサイトにアクセスしてください: www.example.jp。  \n \"これは...簡単ではありません,\" と彼は言いました。 \t 例えば: これは***非常に珍しい問題***です。  ... 明日来ますか？ はい、おそらく（またはいいえ）。");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for javanese.");
    let sentences = processor::javanese("ꦲꦶꦤꦸ ꦧꦸꦱꦸ? 1. ꦥꦿꦺꦴꦒꦼꦠꦶ ꦢꦶꦧꦸ ꦩꦸꦠꦸ. 2. ꦩꦁꦒꦶꦭꦶ ꦱꦶꦠꦺ ꦲꦸꦕꦶꦣꦶ: www.example.com. \n \"ꦯꦶꦠꦸ ꦤꦺꦴ... ꦲꦸꦤꦺꦴ,\" ꦲꦺꦴ ꦯꦸꦏꦸ. \t ꦧꦸꦗꦶꦭꦶ, ꦯꦸꦧꦴ ***ꦥꦿꦸꦭꦁꦒꦶꦣꦶ***. ... ꦲꦶꦪꦴ ꦏꦸꦭꦼꦤꦴ? ꦩꦺꦴꦧꦸ, ꦧꦶꦠꦶ (ꦕꦸꦭꦸ ꦤꦺꦴ). Apa kabar? 1. Proyek harus selesai. 2. Kunjungi situs ini: www.example.com. \n \"Ini tidak... mudah,\" katanya. \t Misalnya, ini adalah ***masalah yang tidak biasa***. ... Apakah kamu akan datang besok? Mungkin, bisa jadi ya (atau tidak).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for kannada.");
    let sentences = processor::kannada("ಹೇಗಿದ್ದೀಯೆ? 1. ಯೋಜನೆ ಪೂರ್ಣಗೊಳ್ಳಬೇಕು. 2. ಈ ವೆಬ್‌ಸೈಟ್‌ಗೆ ಹೋಗಿ: www.example.com. \n \"ಇದು ಸಹಜ... ಅಲ್ಲ,\" ಅವನು ಹೇಳಿದರು. \t ಉದಾಹರಣೆಗೆ, ಇದು ***ಅಸಾಮಾನ್ಯ ಸಮಸ್ಯೆ***. ... ನಾಳೆ ಬರುವಿರಾ? ಬಹುಶಃ, ಸಾಧ್ಯವಿದೆ (ಅಥವಾ ಇಲ್ಲ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for kashmiri.");
    let sentences = processor::kashmiri("چُھھَے؟ 1. منصوبہ مکمل ہونا چاہئیے۔ 2. اس ویب سائٹ پر جائیں: www.example.com۔ \n \"یہ اتنا... آسان نہیں ہے،\" اُس نے کہا۔ \t مثال کے طور پر، یہ ایک ***غیر معمولی مسئلہ*** ہے۔ ... کیا تم کل آؤ گے؟ شاید، ہو سکتا ہے (یا نہیں)۔");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for kazakh.");
    let sentences = processor::kazakh("Qalaysyz? 1. Jobalyq jobany tolyqtandyru kerek. 2. Osyl saytqa kiringiz: www.misal.kz.  \n \"Bul... oson emes,\" dep aytty. \t Mısaly: Bul ***ótewi jerkeïk másele***.  ... Erteñ kelipesiñ be? Iya, bälkim (nemese joq). Қалайсыз? 1. Жобалық жұмысты толықтыру керек. 2. Осы сайтқа кіріңіз: www.misal.kz.  \n \"Бұл... оңай емес,\" деп айтты. \t Мысалы: Бұл ***өте ерекше мәселе***.  ... Ертең келесіз бе? Ия, бәлкім (немесе жоқ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for khmer.");
    let sentences = processor::khmer("សុខសប្បាយរូបរាង? 1. គម្រោងត្រូវតែបញ្ចប់។ 2. សូមចូលទៅកាន់វេបសាយនេះ: www.example.kh.  \n \"នេះ... មិនមែនងាយទេ,\" គាត់បាននិយាយ។ \t ឧទាហរណ៍: នេះគឺជាបញ្ហា ***ដែលមិនធម្មតា***។  ... អ្នកនឹងមកស្អែកទេ? បាទ ប្រហែល (ឬមិនទេ)។");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for korean.");
    let sentences = processor::korean("안녕하세요? 1. 프로젝트는 완료되어야 합니다. 2. 이 웹사이트를 방문하세요: www.example.kr.  \n \"이건... 쉽지 않아요,\" 라고 말했습니다. \t 예를 들어: 이건 ***매우 특이한 문제***입니다.  ... 내일 올 건가요? 네, 아마도 (아니면 아니요).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for kurdish.");
    let sentences = processor::kurdish("Tu çawa yî? 1. Projeya divê bi dawî bibe. 2. Kerema xwe vê malperê bisêre: www.misal.kr.  \n \"Ev... asan nake,\" got. \t Mînaka: Ev pirsgirêk ***gelek negerî*** ye.  ... Tu ê sibehê tê bibî? Erê, heke na (an jî na).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for kyrgyz.");
    let sentences = processor::kyrgyz("Kalaysyz? 1. Proyekt toluq tamamlanyp, bitirilishi kerek. 2. Bul saytqa kirgile: www.example.kg.  \n \"Bul... oson emes,\" dep ayttı. \t Mısaly: Bul ***çok nadir mesele***.  ... Erteñ kelipesiñbe? Iya, bälkim (nemese joq).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for lao.");
    let sentences = processor::lao("ສະບາຍດີບໍ? 1. ການຈັດການຄວນສຳເລັດ. 2. ກະລຸນາເຂົ້າເວັບໄຊນີ້: www.example.la.  \n \"ນີ້... ບໍ່ງ່າຍ,\" ເຂົາກະວາງໄປ. \t ເຫົາຕົວຢ່າງ: ນີ້ເປັນປັນຫາ ***ທີ່ຫຼາຍຢ່າງ***.  ... ທ່ານຈະມາມື້ຫຼັງຫຼືບໍ? ແມ່ນ, ອາດຈະ (ຫຼືບໍ່).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for latin.");
    let sentences = processor::latin("Salve! 1. Proiectum confestim perficitur. 2. Visita hunc locum: www.example.la.  \n \"Hoc... non facile est,\" dixit. \t Exempli gratia: Hoc est ***problema raro***.  ... Cras venies? Ita, fortasse (vel non).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for latvian.");
    let sentences = processor::latvian("Sveiki! 1. Projekts jābeidz. 2. Lūdzu, apmeklējiet šo vietni: www.example.lv.  \n \"Tas... nav viegli,\" viņš teica. \t Piemēram: Tas ir ***ļoti neparasts problēma***.  ... Vai tu nāksi rīt? Jā, iespējams (vai nē).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for lithuanian.");
    let sentences = processor::lithuanian("Sveiki! 1. Projektą reikia užbaigti. 2. Prašome apsilankyti šiame puslapyje: www.example.lt.  \n \"Tai... nėra lengva,\" jis pasakė. \t Pavyzdžiui: Tai yra ***labai retas atvejis***.  ... Ar ateisi rytoj? Taip, galbūt (arba ne).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for macedonian.");
    let sentences = processor::macedonian("Здраво! 1. Проектот треба да се заврши. 2. Ве молиме посетете ја оваа веб-страница: www.example.mk.  \n \"Ова... не е лесно,\" рече тој. \t Пример: Ова е ***многу ретка ситуација***.  ... Ќе дојдеш утре? Да, можеби (или не).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for malagasy.");
    let sentences = processor::malagasy("Manao ahoana ianao? 1. Tokony ho vita ny tetikasa. 2. Tsidiho ity tranokala ity: www.example.com. \n \"Tsy mora... izany,\" hoy izy. \t Ohatra, ity dia ***olana tsy mahazatra***. ... Hiverina ve ianao rahampitso? Angamba, mety ho eny (na tsia).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for malay.");
    let sentences = processor::malay("Hai! 1. Projek ini perlu diselesaikan. 2. Sila lawati laman web ini: www.example.my.  \n \"Ini... tidak mudah,\" kata dia. \t Contohnya: Ini adalah ***masalah yang sangat jarang***.  ... Adakah anda akan datang esok? Ya, mungkin (atau tidak).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for malayalam.");
    let sentences = processor::malayalam("സുഖമാണോ? 1. പദ്ധതി പൂർത്തിയാകണം. 2. ഈ വെബ്‌സൈറ്റ് സന്ദർശിക്കുക: www.example.com. \n \"ഇത് എളുപ്പം... അല്ല,\" അദ്ദേഹം പറഞ്ഞു. \t ഉദാഹരണത്തിന്, ഇത് ***അസാധാരണ പ്രശ്നമാണ്***. ... നാളെ വരാനിരിക്കേ? ആകാം, എന്നാൽ ഇല്ല (അല്ലെങ്കിൽ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for maltese.");
    let sentences = processor::maltese("Kif inti? 1. Il-proġett għandu jitlesta. 2. Żur dan is-sit web: www.example.com. \n \"Dan mhuwiex... faċli,\" qal hu. \t Bħal eżempju, dan hu ***problema mhux komuni***. ... Se tiġi għada? Forse, jista' jkun iva (jew le).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for manipuri.");
    let sentences = processor::manipuri("ꯃꯤꯌꯥ ꯅꯦ? 1. ꯕꯥꯛꯇꯦ ꯆꯩꯕꯥ ꯧꯌꯥꯡꯗꯦ. 2. ꯏꯕꯤ ꯂꯥꯔ ꯱ꯩꯖꯗꯦ: www.example.com. \n \"ꯇꯣꯝ ꯥꯌꯠ... ꯏꯍꯦ,\" ꯱ꯩ ꯐꯦ. \t ꯇꯦꯝꯒꯥ, ꯇꯝ ꯝꯧ ***ꯂꯧꯍꯥꯔꯦꯡ ꯐꯥꯏꯅꯥ***. ... ꯅꯥꯔꯦ ꯋꯥꯇꯥ ꯍꯦ? ꯑꯣꯏ, ꯗꯧꯇꯏꯝ (ꯍꯦ ꯅꯧ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for maori.");
    let sentences = processor::maori("Haere mai! 1. Me oti te kaupapa. 2. Tēnā, toro mai ki tēnei paetukutuku: www.example.nz.  \n \"Aroha mai... kāore e ranea,\" te kī āna. \t Hei tauira: Ko tēnei he ***raru tino rerekē***.  ... Ka tae mai koe āpōpō? Āe, pea (kei te kāore).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for marathi.");
    let sentences = processor::marathi("नमस्कार! 1. हा प्रकल्प पूर्ण करणे आवश्यक आहे. 2. कृपया या वेबसाइटला भेट द्या: www.example.in.  \n \"हे... सोपे नाही,\" त्याने सांगितले. \t उदाहरणार्थ: हे ***खूप दुर्मिळ आहे***.  ... तुम्ही उद्या येणार का? हो, कदाचित (किंवा नाही).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for mongolian.");
    let sentences = processor::mongolian("Сайн уу! 1. Төслийг дуусгах хэрэгтэй. 2. Энэ вэбсайтыг үзнэ үү: www.example.mn.  \n \"Энэ... амар биш,\" гэж тэр хэлэв. \t Жишээ нь: Энэ бол ***маш ховор асуудал***.  ... Та маргааш ирэх үү? Тийм, магадгүй (эсвэл үгүй).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for nepali.");
    let sentences = processor::nepali("नमस्ते! 1. यो परियोजना पूरा गर्नु पर्छ। 2. कृपया यस वेबसाइटमा जानुहोस्: www.example.com.  \n \"यो... सजिलो छैन,\" उसले भन्यो। \t उदाहरणका लागि: यो ***दुर्लभ समस्या हो***।  ... के तपाईं भोलि आउनेछौं? हो, शायद (वा होइन).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for norwegian.");
    let sentences = processor::norwegian("Hei! 1. Prosjektet må fullføres. 2. Vennligst besøk denne nettsiden: www.example.no.  \n \"Dette... er ikke lett,\" sa han. \t For eksempel: Dette er ***et veldig sjeldent tilfelle***.  ... Kommer du i morgen? Ja, kanskje (eller ikke).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for oriya.");
    let sentences = processor::oriya("କେମିତି ଅଛନ୍ତି? 1. ପରିୟୋଜନା ଶେଷ ହେବା ଚାହିଁଦି। 2. ଏହି ୱେବସାଇଟ୍ ଦେଖନ୍ତୁ: www.example.com। \n \"ଏହା ଏତେ... ସହଜ ନୁହେଁ,\" ସେ କହିଲେ। \t ଉଦାହରଣ ସ୍ୱରୂପ, ଏହା ଏକ ***ଅସାଧାରଣ ସମସ୍ୟା***। ... କାଲି ଆସିବେ କି? ସମ୍ଭାବନା ଅଛି, ହଁ (କିମ୍ବା ନୁହେଁ)।");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for ossetian.");
    let sentences = processor::ossetian("Хъæуы иу? 1. Проект æвдæмæй хуыдты. 2. Фæссæйтæн адæмæ: www.example.com. \n \"Æнæзæн... æсгæу,\" хъуыдты. \t Уæдæу, æйы ***гæнзæн фæдæг***. ... Бæрæнтæй хъæуы? Сæвæд, æнæн фæрæ (æнæн).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for pashto.");
    let sentences = processor::pashto("سلام! 1. دغه پروژه بايد بشپړه شي. 2. لطفاً دې ويب پاڼې ته ورشئ: www.example.af.  \n \"دا... آسانه ندی,\" هغه وویل. \t د بېلګې په توګه: دا یو ***ډیر نادره ستونزه*** ده.  ... آیا تاسو سبا راځي؟ هو، ښايي (یا نه).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for persian.");
    let sentences = processor::persian("سلام! 1. این پروژه باید کامل شود. 2. لطفاً به این وب‌سایت مراجعه کنید: www.example.ir.  \n \"این... آسان نیست,\" او گفت. \t به عنوان مثال: این یک ***مسئله بسیار نادر*** است.  ... آیا فردا می‌آیی؟ بله، شاید (یا نه).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for polish.");
    let sentences = processor::polish("Cześć! 1. Ten projekt musi zostać ukończony. 2. Proszę odwiedzić tę stronę: www.example.pl.  \n \"To... nie jest łatwe,\" powiedział. \t Na przykład: To ***bardzo rzadki problem***.  ... Czy przyjdziesz jutro? Tak, może (albo nie).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for portuguese.");
    let sentences = processor::portuguese("Olá! 1. Este projeto precisa ser concluído. 2. Por favor, visite este site: www.example.pt.  \n \"Isso... não é fácil,\" ele disse. \t Por exemplo: Este é um ***problema muito raro***.  ... Você virá amanhã? Sim, talvez (ou não).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for eastern punjabi.");
    let sentences = processor::punjabi_eastern("ਸਤ ਸ੍ਰੀ ਅਕਾਲ! 1. ਇਸ ਪ੍ਰੋਜੈਕਟ ਨੂੰ ਪੂਰਾ ਕਰਨਾ ਲੋੜੀਦਾ ਹੈ। 2. ਕਿਰਪਾ ਕਰਕੇ ਇਸ ਵੈਬਸਾਈਟ ਤੇ ਜਾਓ: www.example.in.  \n \"ਇਹ... ਆਸਾਨ ਨਹੀਂ ਹੈ,\" ਉਸਨੇ ਕਿਹਾ। \t ਉਦਾਹਰਣ ਵਜੋਂ: ਇਹ ***ਬਹੁਤ ਹੀ ਦੁਖਦਾਈ ਸਮੱਸਿਆ*** ਹੈ।  ... ਕੀ ਤੁਸੀਂ ਕੱਲ੍ਹ ਆਓਗੇ? ਹਾਂ, ਸ਼ਾਇਦ (ਜਾਂ ਨਹੀਂ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for western punjabi.");
    let sentences = processor::punjabi_western("سلام! 1. اِس پراجیکٹ کو مکمل کرنا ضروری ہے۔ 2. براہ کرم اس ویب سائٹ پر جائیں: www.example.pk.  \n \"یہ... آسان نہیں ہے،\" اُس نے کہا۔ \t مثال کے طور پر: یہ ***بہت نادر مسئلہ*** ہے۔  ... کیا آپ کل آئیں گے؟ ہاں، شاید (یا نہیں)۔");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for romanian.");
    let sentences = processor::romanian("Bună! 1. Acest proiect trebuie finalizat. 2. Te rog să vizitezi acest site: www.example.ro.  \n \"Acesta... nu este ușor,\" a spus el. \t De exemplu: Aceasta este o ***problemă foarte rară***.  ... Vii mâine? Da, poate (sau nu).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for russian.");
    let sentences = processor::russian("Привет! 1. Этот проект необходимо завершить. 2. Пожалуйста, зайдите на этот сайт: www.example.ru.  \n \"Это... не так просто,\" — сказал он. \t Например: Это ***очень редкая проблема***.  ... Ты придёшь завтра? Да, может быть (или нет).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for sanskrit.");
    let sentences = processor::sanskrit("कथमस्ति भवान्? 1. परियोजना समाप्तव्या। 2. अस्मिन वेबसाइटे गच्छतु: www.example.com। \n \"एषः अतिविशिष्टः... न सोऽस्ति,\" सः अवदत्। \t उदाहरणार्थ, एषः ***विशिष्ट समस्या अस्ति***। ... गते दिने आगच्छसि वा? कदाचित्, यद्यपि न (किं वा)?");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for santali.");
    let sentences = processor::santali("ᱯᱷᱚᱯᱟᱯᱤ ᱟᱢᱮ? 1. ᱯᱤᱡᱟᱹᱴᱟ ᱵᱚᱸᱰᱟᱭᱟᱜᱤ. 2. ᱟᱞᱤᱜ ᱜᱷᱩᱥᱮᱸ ᱵᱷᱤᱰᱷᱩᱷᱤᱞ: www.example.com. \n \"ᱚᱝᱚ ᱟᱢᱮ... ᱯᱟᱹᱜᱤ,\" ᱟᱠᱮᱢᱮ. \t ᱵᱷᱚᱯᱟᱯᱤ, ᱟᱝᱚ ᱮᱥᱮ ***ᱪᱷᱟᱜᱮᱡ ᱛᱤᱱᱟᱜᱟ***. ... ᱴᱷᱟᱝᱤᱧ ᱯᱷᱩᱯᱤᱝ? ᱛᱚᱸᱰᱮ, ᱛᱮᱱᱴᱤᱝ (ᱛᱮᱱᱴᱤᱝ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for scottish.");
    let sentences = processor::scottish("Hoozit! 1. This projeck needs tae be feenished. 2. Gae tae this wabsteid: www.example.scot.  \n \"It's... no that easy,\" he said. \t For ensample: ***This is a gey rare problem***.  ... Will ye come the morn? Aye, mebbe (or naw).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for serbian.");
    let sentences = processor::serbian("Zdravo! 1. Ovaj projekat mora biti završen. 2. Molim te, poseti ovu stranicu: www.example.rs.  \n \"Ovo... nije lako,\" rekao je. \t Na primer: Ovo je ***veoma redak problem***.  ... Dolaziš sutra? Da, možda (ili ne). Здраво! 1. Овај пројекат мора бити завршен. 2. Молим те, посети ову страницу: www.example.rs.  \n \"Ово... није лако,\" рекао је. \t На пример: Ово је ***веома редак проблем***.  ... Долазиш сутра? Да, можда (или не).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for shona.");
    let sentences = processor::shona("Mhoro! 1. Basa iri rinofanira kupedzwa. 2. Ndapota, shanyira webhusaiti iyi: www.example.zw.  \n \"Izvi... hazvisi nyore,\" akadaro. \t Semuenzaniso: ***Iri idambudziko rakakosha zvikuru***.  ... Uchauya mangwana? Ehe, pamwe (kana kwete).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for sindhi.");
    let sentences = processor::sindhi("कयें आहे? 1. प्रकल्प पूर्ण व्हावा लागेल. 2. या वेबसाइटला पहा: www.example.com. \n \"हे इतके... सोपे नाही,\" त्याने सांगितले. \t उदाहरणार्थ, हे ***असाधारण समस्या*** आहे. ... उद्या येणार का? कदाचित, हो (किंवा नाही). ڪئن آهي؟ 1. پروجيڪٽ مڪمل ٿيڻ گهرجي. 2. هن ويب سائيٽ کي ڏسو: www.example.com. \n \"اهو ايترو... سولو ناهي،\" هن چيو. \t مثال طور، اهو ***غير معمولي مسئلو*** آهي. ... سڀاڻي ايندين؟ شايد، ٿي سگهي ٿو (يا نه).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for sinhala.");
    let sentences = processor::sinhala("හෙලෝ! 1. මේ ව්‍යාපෘතිය අවසන් කළ යුතුයි. 2. කරුණාකර මෙම වෙබ් අඩවිය වෙත පිවිසෙන්න: www.example.lk.  \n \"මෙය... පහසු නැහැ,\" ඔහු කිවුවා. \t උදාහරණයක් වශයෙන්: ***මේක ඉතා අධික ගැටළුවක්***.  ... ඔබ හෙට එනවාද? ඔව්, සමහර විට (නැති නම් නැහැ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for slovak.");
    let sentences = processor::slovak("Ahoj! 1. Tento projekt musí byť dokončený. 2. Prosím, navštívte túto stránku: www.example.sk.  \n \"Toto... nie je jednoduché,\" povedal. \t Napríklad: ***Toto je veľmi zriedkavý problém***.  ... Prídeš zajtra? Áno, možno (alebo nie).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for slovenian.");
    let sentences = processor::slovenian("Živjo! 1. Ta projekt je treba dokončati. 2. Prosimo, obiščite to spletno stran: www.example.si.  \n \"To... ni preprosto,\" je rekel. \t Na primer: ***To je zelo redek problem***.  ... Ali prideš jutri? Ja, morda (ali pa ne).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for somali.");
    let sentences = processor::somali("Salaan! 1. Mashruucan waa in la dhammeeyaa. 2. Fadlan booqo boggan: www.example.so.  \n \"Tani... ma fududa,\" ayuu yiri. \t Tusaale ahaan: ***Tani waa dhibaato aad u weyn***.  ... Ma imanaysaa berrito? Haa, laga yaabee (ama maya).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for soto.");
    let sentences = processor::sotho("Lumela! 1. Morero ona o tlameha ho phethoa. 2. Ka kopo etela sebaka sena sa marang-rang: www.example.ls.  \n \"Sena... ha se bonolo,\" a rialo. \t Mohlala: ***Sena ke bothata bo boholo haholo***.  ... Na u tla tla hosane? E, mohlomong (kapa che).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for spanish.");
    let sentences = processor::spanish("¡Hola! 1. Este proyecto debe ser completado. 2. Por favor, visita este sitio web: www.example.es.  \n \"Esto... no es fácil,\" dijo. \t Por ejemplo: ***Este es un problema bastante raro***.  ... ¿Vas a venir mañana? Sí, tal vez (o no).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for sundanese.");
    let sentences = processor::sundanese("كيف حالك؟ 1. يجب إتمام المشروع. 2. زر هذا الموقع: www.example.com. \n \"هذا ليس... سهلاً\"، قال. \t على سبيل المثال، هذه ***مشكلة غير عادية***. ... هل ستأتي غداً؟ ربما، قد لا (أو نعم).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for swahili.");
    let sentences = processor::swahili("Habari! 1. Mradi huu unapaswa kukamilika. 2. Tafadhali tembelea tovuti hii: www.example.co.ke.  \n \"Hii... sio rahisi,\" alisema. \t Kwa mfano: ***Hii ni shida nadra sana***.  ... Je, utakuja kesho? Ndiyo, labda (au hapana).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for swedish.");
    let sentences = processor::swedish("Hej! 1. Detta projekt måste slutföras. 2. Vänligen besök denna webbplats: www.example.se.  \n \"Det här... är inte lätt,\" sade han. \t Till exempel: ***Det här är ett mycket ovanligt problem***.  ... Kommer du imorgon? Ja, kanske (eller inte).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for tagalog.");
    let sentences = processor::tagalog("Kamusta! 1. Ang proyektong ito ay kailangang tapusin. 2. Paki-bisita ang website na ito: www.example.ph.  \n \"Ito... ay hindi madali,\" sabi niya. \t Halimbawa: ***Ito ay isang bihirang problema***.  ... Darating ka ba bukas? Oo, marahil (o hindi).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for tamil.");
    let sentences = processor::tamil("வணக்கம்! 1. இந்த திட்டத்தை முடிக்க வேண்டும். 2. தயவுசெய்து இந்த வலைத்தளத்தை பார்வையிடுக: www.example.in.  \n \"இதன்... மிக எளிதாக இல்லை,\" அவர் சொன்னார். \t உதாரணமாக: ***இது ஒரு அரிதான பிரச்சினை***.  ... நீ எதிர்காலம் நாளை வருவாயா? ஆம், ஒருவேளை (அல்லது இல்லை).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for tatar.");
    let sentences = processor::tatar("Ничек сез? 1. Проект тәмамланырга тиеш. 2. Бу вебсайтны карагыз: www.example.com. \n \"Бу бик... җиңел түгел,\" диде ул. \t Мисал өчен, бу ***гадәти булмаган мәсьәлә***. ... Иртәгә килерсеңме? Бәлки, килмәссең (яисә әйе).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for telugu.");
    let sentences = processor::telugu("హలో! 1. ఈ ప్రాజెక్టును పూర్తిచేయాలి. 2. దయచేసి ఈ వెబ్‌సైట్‌ను సందర్శించండి: www.example.in.  \n \"ఇది... సులభం కాదు,\" అని అతను చెప్పాడు. \t ఉదాహరణకు: ***ఇది ఒక అరుదైన సమస్య***.  ... మీరు రేపు రా? అవును, బహుశా (లేదా కాదు).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for tibetan.");
    let sentences = processor::tibetan("བཀྲ་ཤིས་། ཁྱེད་ཀྱིས་ག་རེ་འདུག? 1. གྲོལ་འགོ་བའི་འབྲེལ་བ་འགུལ་བ་འདུག. 2. འདི་ནང་ཡིག་འཕྲིན་འདོན་འགུལ་འདུག: www.example.com. \n \"འདུག་པ་ལས... སྤྱིར་ན་མེད།\" འདུག་ངེས། \t བརྡེད་ལས, འདུག ***མི་འདུག་པ་དེ་འགྲོལ་མེད།***. ... ཉིན་ལ་འགྲོལ་གུག? ངག་མཐའ་དུ, ང་མི་འདུག (ཡང་ཡོང).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for tsonga.");
    let sentences = processor::tsonga("Xewani! 1. Leswi swi fanele ku hetiseka. 2. Hikwalaho, u fanele u vona website leyi: www.example.co.za.  \n \"Leswi... a swi olovi,\" u byile. \t Beispel: ***Leswi i nkqubo ya xifundza***.  ... U ta ta swi njhani? E-e, hinkwavo ( kumbe a ku na swona).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for tswana.");
    let sentences = processor::tswana("Dumelang! 1. Lenaneo leno le tshwanetse go fetela pele. 2. Ka kopo, etela webosaete eno: www.example.co.za.  \n \"Se... ga se bonolo,\" o buile. \t Mohlala: ***Se ke bothata bo sa tloaelehang***.  ... O tla tla ka letsatsi la gompieno? Ee, go ka ba teng (goba ga go teng).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for turkish.");
    let sentences = processor::turkish("Merhaba! 1. Bu projeyi tamamlamalıyız. 2. Lütfen şu web sitesini ziyaret edin: www.example.tr.  \n \"Bu... kolay değil,\" dedi. \t Örnek: ***Bu nadir bir sorun***.  ... Yarın gelir misin? Evet, belki (ya da hayır).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for turkmen.");
    let sentences = processor::turkmen("Näme ýagdaýda? 1. Taslama tamamlanmalydyr. 2. Bu web sahypasyny gör: www.example.com. \n \"Bu aňsat däl,\" diýdi ol. \t Mysal üçin, bu ***adaty bolmadyk mesele***. ... Ertir gelýärsiňmi? Belki, gelmezsiň (ýa-da howa).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for ukrainian.");
    let sentences = processor::ukrainian("Привіт! 1. Цей проєкт потрібно завершити. 2. Будь ласка, відвідайте цей вебсайт: www.example.ua.  \n \"Це... не так легко,\" сказав він. \t Приклад: ***Це рідкісна проблема***.  ... Ти прийдеш завтра? Так, можливо (або ні).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for urdu.");
    let sentences = processor::urdu("ہیلو! 1. اس منصوبے کو مکمل کرنا ضروری ہے۔ 2. براہ کرم اس ویب سائٹ پر جائیں: www.example.pk.  \n \"یہ... آسان نہیں ہے،\" اس نے کہا۔ \t مثال: ***یہ ایک نادر مسئلہ ہے***۔  ... کیا آپ کل آئیں گے؟ ہاں، شاید (یا نہیں)۔");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for uyghur.");
    let sentences = processor::uyghur("قايناق؟ 1. پىلان تاماملىنىشى كېرەك. 2. بۇ تور بېكەتكە كىرىڭ: www.example.com. \n \"بۇ بەك... ئاسان ئەمەس\"، دەپ ئىپادىلەندى. \t مەسىلەن، بۇ ***ئادەتتىكى بولمىغان مەسىلە***. ... ئەتىرەڭ كەلەمۇ؟ بەلکی، كەلمەسسىڭ (ياكى بولىدۇ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for uzbek.");
    let sentences = processor::uzbek("Salom! 1. Ushbu loyihani tugatish kerak. 2. Iltimos, ushbu vebsaytga o'ting: www.example.uz.  \n \"Bu... oson emas,\" dedi u. \t Misol: ***Bu kam uchraydigan muammo***.  ... Ertaga kelasizmi? Ha, ehtimol (yoki yo'q).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for vietnamese.");
    let sentences = processor::vietnamese("Chào bạn! 1. Dự án này cần phải hoàn thành. 2. Vui lòng truy cập vào trang web: www.example.vn.  \n \"Điều này... không dễ dàng,\" anh ấy nói. \t Ví dụ: ***Đây là một vấn đề hiếm gặp***.  ... Bạn sẽ đến vào ngày mai? Có thể, có thể không (hoặc không).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for volapuk.");
    let sentences = processor::volapuk("Köpäï?! 1. Un proyekto iksölemás. 2. Çi wébside ësta: www.example.com. \n \"Për ësö... no es asó,\" nífes tü. \t Pəsïlë, ke ***bolëa niwöbada***. ... Takatalä?! Çizá! (Oi nu).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for welsh.");
    let sentences = processor::welsh("Helo! 1. Mae'n rhaid i ni gwblhau'r prosiect hwn. 2. Ewch i'r wefan hon: www.example.co.uk.  \n \"Mae hyn... yn anodd,\" meddai. \t Enghraifft: ***Mae hyn yn broblem prin***.  ... A fyddwch chi'n dod yfory? Ie, efallai (neu beidio).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for xhosa.");
    let sentences = processor::xhosa("Molo! 1. Kufuneka siphelise le projekthi. 2. Nceda uhambe kwiwebhusayithi: www.example.co.za.  \n \"Oku... akulula,\" utshilo. \t Umzekelo: ***Le yimeko enqamleqayo***.  ... Uzakufika kusasa? Ewe, mhlawumbi (okanye hayi).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for yiddish.");
    let sentences = processor::yiddish("העלא! 1. מיר מוזן פֿאַרענדיקן דעם פּראָיעקט. 2. ביטע גיי אויף דער וועבזייטל: www.example.com.  \n \"דאס... איז נישט אָן שווער,\" ער האָט געזאָגט. \t ביישפּיל: ***דאָס איז אַ געלונגענע צעשלעכונג***.  ... וועסטו קומען מאָרגן? יאָ, אפֿשר (אָדער נישט).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for yoruba.");
    let sentences = processor::yoruba("Bawo! 1. A gbọdọ pari iṣẹ́ akanṣe yìí. 2. Jọwọ lọ si oju opo wẹẹbu: www.example.com.  \n \"Eyi... kò rọọrun,\" ni ó sọ. \t Àpẹẹrẹ: ***Èyí jẹ́ iṣoro tí kò wọpọ***.  ... Ṣé iwọ yoo wá lọ́la? Béẹ ni, bóyá (tàbí bẹ́ẹ̀ kọ).");
    for sentence in sentences {
        println!("{}", sentence);
    }

    println!("\nTesting sentence segmentation for zulu.");
    let sentences = processor::zulu("Sawubona! 1. Kufanele siqedele le phrojekthi. 2. Sicela uvakashele iwebhusayithi: www.example.co.za.  \n \"Lokhu... akulula,\" kusho yena. \t Isibonelo: ***Lokhu kuyinkinga evamile***.  ... Uzofika kusasa? Yebo, mhlawumbe (noma cha).");
    for sentence in sentences {
        println!("{}", sentence);
    }
}