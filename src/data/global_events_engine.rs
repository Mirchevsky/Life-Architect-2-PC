/// Global Events Engine — ported from the Android GlobalEventsEngine.kt
///
/// Architecture:
///  - All 365 event entries are embedded in this file (one per calendar day).
///  - Selection is purely date-based (MM-dd key) — no rotation state needed.
///  - Year wrap (Dec 31 → Jan 1) is handled automatically by chrono.
///  - Falls back to a safe placeholder if the key is somehow missing.

use chrono::{Datelike, Local, NaiveDate};
use std::collections::HashMap;

// ── Public types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GlobalEvent {
    pub title: String,
    pub description: String,
}

// ── Engine ───────────────────────────────────────────────────────────────────

/// Returns the complete event map (MM-dd → GlobalEvent).
fn build_event_map() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut m = HashMap::new();

    // January
    m.insert("01-01", ("New Year's Day", "New Year's marks the beginning of a new calendar year and is often associated with fresh goals, starting the celebrations. Around the world, people use the day to reflect on the past and look ahead to new possibilities."));
    m.insert("01-02", ("Science Fiction Day", "Today, we celebrate imaginative stories that explore advanced technological unknown worlds. It is a day to enjoy the creativity of a genre that often inspires real innovation."));
    m.insert("01-03", ("Drinking Straw Day", "Today, we recognize the everyday invention that made sipping beverages convenient. It is a lighthearted reminder that even simple tools can have a lasting impact on daily life."));
    m.insert("01-04", ("World Braille Day", "Today highlights the importance of Braille as a reading and writing system for visually impaired people. The day promotes accessible equal access to information."));
    m.insert("01-05", ("Bird Day", "Today is a chance to appreciate the important diversity of birds in nature. It also encourages awareness of the conservation of bird habitats around the world."));
    m.insert("01-06", ("Technology Day", "Today, we celebrate the system tools as the inventions that shape modern life. It is a great occasion to reflect on how technology improves aspects of our lives."));
    m.insert("01-07", ("Bobblehead Day", "This day is a playful celebration of the collectible figurines known for their oversized heads of nodding motion. Let's embrace the fun of quirky pop culture."));
    m.insert("01-08", ("Bubble Bath Day", "This day is all about being comfortable by taking the time to unwind. Turn an ordinary relaxation routine into a cheerful excuse to slow down by enjoying a peaceful moment."));
    m.insert("01-09", ("Balloon Ascension Day", "Today commemorates the early era of human flight surrounding hot air balloons. It reflects the adventurous spirit of exploration and our ancestors' dream of rising above the ground."));
    m.insert("01-10", ("Houseplant Appreciation Day", "Today, we celebrate the beautiful benefits of indoor plants. It is a reminder that plants can bring a touch of nature into everyday life."));
    m.insert("01-11", ("International Thank You Day", "Today, encourage people to express gratitude to others in a simple but meaningful way. A sincere thank you can strengthen relationships through positive everyday interactions."));
    m.insert("01-12", ("National Pharmacist Day", "This day recognizes the people who help others consume medicine safely. It highlights care by providing the knowledge of the important role pharmacists play in everyday health."));
    m.insert("01-13", ("Sticker Day", "Today, the creative fun of stickers begins, from decorations to expressions. Whether you use it for art or organizations, stickers have long been a simple source of joy."));
    m.insert("01-14", ("World Logic Day", "Today, we recognize the value of logic through everyday critical thinking. We celebrate reason with a clear truth that the role of logic is to understand the world."));
    m.insert("01-15", ("Hat Day", "Today, let's celebrate one of the most practical and stylish accessories in human culture. Hats can provide an expression of personality that reflects traditions across many places."));
    m.insert("01-16", ("National Good Teen Day", "Today, teens show responsible kindness as a positive character trait. It is a day to appreciate young people who make a good impact on those around them."));
    m.insert("01-17", ("Classy Day", "Today, we celebrate graceful elegance with good manners. Encourage people to appreciate refinement not only in style, but also in how they treat others."));
    m.insert("01-18", ("Thesaurus Day", "Today, we celebrate the richness of language with all the many ways words can express meaning. It encourages curiosity about vocabulary that reminds us how word choices can shape communication."));
    m.insert("01-19", ("Elementary School Teacher Day", "This day honors teachers who help children grow to learn and build confidence at an early age, which also celebrates patience, which guides the lasting influence of great educators."));
    m.insert("01-20", ("Penguin Awareness Day", "Today, the world celebrates penguins and encourages people to learn more about these unique birds. It also raises appreciation for the wildlife environments penguins depend on."));
    m.insert("01-21", ("Hugging Day", "Today, you can celebrate the warmth of an affectionate human connection. Hugs provide a gentle reminder of the power of simple kindness."));
    m.insert("01-22", ("Celebration of Life Day", "Today encourages people to appreciate life's relationships as meaningful moments. It is a positive occasion to show gratitude by reflecting joy."));
    m.insert("01-23", ("Handwriting Day", "This day and age highlights the personal expressive nature of writing by hand. In the digital age, today is a reminder of the creative individuality found in handwritten words."));
    m.insert("01-24", ("International Day of Education", "This day emphasizes the importance of learning as a foundation for equal opportunity. It recognizes education as a powerful growth force for our personal and social development."));
    m.insert("01-25", ("Opposite Day", "Today is a playful day built around reversing expectations by doing the opposite of what is usual, so add some unpredictable humor to everyday routines."));
    m.insert("01-26", ("International Day of Clean Energy", "This day highlights the importance of sustainable energy sources for the future of the planet. It draws attention to cleaner alternatives that support development while reducing environmental harm."));
    m.insert("01-27", ("International Holocaust Remembrance Day", "Today, we honor the memory of the millions of victims of the Holocaust. It is a day of educational remembrance that reflects on the dangers of antisemitic hatred."));
    m.insert("01-28", ("Data Protection Day", "This day raises awareness about digital privacy rights through the responsible handling of personal information. Encourage individuals and organizations to value the security of sensitive data."));
    m.insert("01-29", ("Puzzle Day", "Today celebrates critical thinking through the curiosity of putting pieces together. Whether you prefer brainteasers or not, challenge your mind in a rewarding way."));
    m.insert("01-30", ("World Neglected Tropical Diseases Day", "This day shines a light on diseases that affect millions of people, often in underserved communities. Also, today promotes actionable awareness that supports better prevention treatments in global health equity."));
    m.insert("01-31", ("Inspire Your Heart With Art Day", "Today celebrates the emotional power of art to move and uplift us. It is a reminder that creativity can speak to the heart in ways that words sometimes cannot."));

    // February
    m.insert("02-01", ("Change Your Password Day", "This day reminds people to protect their personal information online. It encourages simple digital safety habits by highlighting the importance of staying secure in a connected world."));
    m.insert("02-02", ("World Wetlands Day", "Today raises awareness about the importance of wetlands such as swamps and lakes. These ecosystems support wildlife by helping to manage water in its major role in the health of the planet."));
    m.insert("02-03", ("National Golden Retriever Day", "Today, we celebrate a dog breed known for its loyal friendliness. It is a cheerful day for appreciating the joy in companionship that dogs bring into people's lives."));
    m.insert("02-04", ("World Cancer Day", "Today promotes prevention awareness that supports people affected by cancer. It encourages education for the global efforts to improve healthcare."));
    m.insert("02-05", ("National Weatherperson's Day", "This day recognizes the people who report the weather. It celebrates the science skills that involve public service in helping communities prepare for changing conditions."));
    m.insert("02-06", ("Pay a Compliment Day", "Today encourages us to brighten someone's day with sincerely kind words. It reminds us that even a small compliment can make a meaningful difference."));
    m.insert("02-07", ("Send a Card to a Friend Day", "Today encourages thoughtful communication by small acts of friendship. A simple message can brighten someone's day while simultaneously strengthening personal connections."));
    m.insert("02-08", ("Kite Flying Day", "This day celebrates a timeless outdoor activity that combines avionics and fun. It is a reminder of the joy that can come from a simple play in the open skies."));
    m.insert("02-09", ("Pizza Day", "Today, we honor one of the world's most popular foods, loved for its variety. It is a fun occasion to celebrate a meal that brings people together."));
    m.insert("02-10", ("All the News That's Fit to Print Day", "This day celebrates the role of honest journalism through the value of sharing reliable information with the public. It is a day that highlights the importance of an informed society."));
    m.insert("02-11", ("International Day of Women and Girls in Science", "This day celebrates the contributions of women and girls in science and encourages opportunities in scientific fields. It highlights the importance of their curious discoveries for innovation."));
    m.insert("02-12", ("Darwin Day", "This day honors the legacy of Charles Darwin and the spirit of scientific curiosity. It is a day to appreciate evidenced based discovery to study life."));
    m.insert("02-13", ("World Radio Day", "Today, we celebrate radio as a powerful tool for information. It recognizes the way radio connects communities and lets us communicate across borders."));
    m.insert("02-14", ("Valentine's Day", "This day celebrates love and affection. While often associated with romance, it can also be a day to show appreciation to all loved ones."));
    m.insert("02-15", ("International Childhood Cancer Day", "Today raises awareness about cancer affecting children and the importance of research. It is a day centered on helping young patients have a better outcome."));
    m.insert("02-16", ("Do a Grouch a Favor Day", "This playful day encourages kindness toward someone who may be having a rough day. It reminds us that generosity can sometimes soften even the grumpiest mood."));
    m.insert("02-17", ("Random Acts of Kindness Day", "Today, try to celebrate small, thoughtful gestures that can make a big difference. It encourages us to spread positivity through acts of goodwill."));
    m.insert("02-18", ("Battery Day", "This day recognizes the invention that powers countless devices in everyday life. It is a chance to appreciate the role of stored energy in technology."));
    m.insert("02-19", ("Prevent Plagiarism Day", "This day encourages being entitled to credit for originality. It highlights the value of learning to respect other people's efforts."));
    m.insert("02-20", ("Love Your Pet Day", "Today is all about appreciating the animals that bring us companionship. It encourages extra care and affection for pets of all kinds."));
    m.insert("02-21", ("International Mother Language Day", "This day highlights the value of linguistic diversity in cultural heritage. It reminds us that language is deeply connected to the community's historical identity."));
    m.insert("02-22", ("Be Humble Day", "Today encourages modesty for yourself and others. It is a gentle reminder that proudless confidence often leaves the strongest impression."));
    m.insert("02-23", ("National Play Tennis Day", "This day celebrates an active sport that combines focused movement and fun. It encourages us to enjoy healthy exercises with a friendly competition."));
    m.insert("02-24", ("National Trading Card Day", "Today celebrates collecting and sharing favorite cards. It highlights the importance of having hobbies through the joy of discovering something meaningful in a small item."));
    m.insert("02-25", ("Inconvenience Yourself Day", "This day encourages you to step beyond what is easy in order to grow. It is a reminder that meaningful progress often comes from outside your comfort zone by doing a little extra."));
    m.insert("02-26", ("Carpe Diem Day", "This day was inspired by the idea of making the most of the present moment. It encourages people to act with appreciation for the power of the now and stop waiting for the perfect time."));
    m.insert("02-27", ("International Polar Bear Day", "This day raises awareness about the environments in which polar bears roam. It encourages learning about wildlife habitats while exploring the world."));
    m.insert("02-28", ("National Time Refund Day", "Today is about using extra time to focus on what matters the most. It encourages us to step back from the usual rush and enjoy ourselves."));
    m.insert("02-29", ("Leap Day", "Leap Day occurs only once every four years, making it a rare and special occasion. It is a day to celebrate the quirks of our calendar and embrace the extraordinary."));

    // March
    m.insert("03-01", ("Zero Discrimination Day", "Today promotes equality, a fairness that respects every person. It encourages communities to reject prejudice to create a world where everyone can live with dignity."));
    m.insert("03-02", ("Old Stuff Day", "Today is a more lighthearted occasion to appreciate things from the past, whether it's objects or memories. It reminds us that older things can still hold a valuable charm."));
    m.insert("03-03", ("World Wildlife Day", "Today celebrates the diversity of wild animals and plants on our planet. It encourages awareness and action to protect the species that share our world."));
    m.insert("03-04", ("National Grammar Day", "This day celebrates the rules and structure that make language clear and effective. It encourages us to appreciate the precision that good grammar brings to communication."));
    m.insert("03-05", ("Multiple Personality Day", "This day encourages awareness and understanding of dissociative identity disorder. It is a reminder to approach mental health topics with empathy and openness."));
    m.insert("03-06", ("National Dentist's Day", "Today recognizes the dental professionals who help keep our smiles healthy. It is a reminder of the importance of oral health in overall wellbeing."));
    m.insert("03-07", ("National Be Heard Day", "This day encourages people to speak up and make their voices count. It is a reminder that every person has something valuable to contribute."));
    m.insert("03-08", ("International Women's Day", "Today celebrates the social, economic, cultural, and political achievements of women. It also marks a call to action for accelerating gender equality."));
    m.insert("03-09", ("Barbie Day", "This day celebrates the iconic doll that has inspired imagination and creativity for generations. It reflects on the cultural impact of play and storytelling."));
    m.insert("03-10", ("International Day of Awesomeness", "Today encourages everyone to do something awesome. It is a lighthearted reminder that greatness can be found in everyday actions."));
    m.insert("03-11", ("World Plumbing Day", "This day recognizes the vital role that plumbing plays in public health and sanitation. It highlights the importance of clean water access for communities worldwide."));
    m.insert("03-12", ("Plant a Flower Day", "Today encourages us to add beauty to our surroundings by planting flowers. It is a reminder of the joy that comes from nurturing living things."));
    m.insert("03-13", ("National Good Samaritan Day", "This day celebrates acts of kindness toward strangers. It encourages us to look out for one another and lend a helping hand when needed."));
    m.insert("03-14", ("Pi Day", "Today celebrates the mathematical constant pi (3.14...). It is a fun occasion for math enthusiasts and a reminder of the beauty found in numbers."));
    m.insert("03-15", ("World Consumer Rights Day", "This day raises awareness about consumer rights and the need for fair markets. It encourages people to know their rights and advocate for fair treatment."));
    m.insert("03-16", ("National Panda Day", "Today celebrates the beloved giant panda and raises awareness about conservation efforts. It is a reminder of the importance of protecting endangered species."));
    m.insert("03-17", ("St. Patrick's Day", "This day celebrates Irish culture and heritage. It is a festive occasion marked by parades, green attire, and a spirit of community."));
    m.insert("03-18", ("Global Recycling Day", "Today highlights the importance of recycling in preserving our planet's resources. It encourages individuals and communities to reduce waste and think sustainably."));
    m.insert("03-19", ("National Let's Laugh Day", "This day celebrates the power of laughter to uplift and connect people. It is a reminder that humor can be a wonderful tool for wellbeing."));
    m.insert("03-20", ("International Day of Happiness", "Today recognizes happiness as a fundamental human goal. It encourages people to reflect on what brings joy and to spread positivity to others."));
    m.insert("03-21", ("World Poetry Day", "This day celebrates the art of poetry and its power to express the human experience. It encourages reading, writing, and sharing poems."));
    m.insert("03-22", ("World Water Day", "Today raises awareness about the global water crisis and the importance of clean water. It calls for action to ensure sustainable water management for all."));
    m.insert("03-23", ("World Meteorological Day", "This day celebrates the science of weather and climate. It highlights the importance of meteorology in understanding and preparing for environmental changes."));
    m.insert("03-24", ("World Tuberculosis Day", "Today raises awareness about the global impact of tuberculosis and efforts to end the disease. It highlights the importance of prevention, diagnosis, and treatment."));
    m.insert("03-25", ("International Waffle Day", "This day celebrates the beloved waffle in all its delicious forms. It is a fun occasion to enjoy a classic treat and appreciate simple pleasures."));
    m.insert("03-26", ("National Spinach Day", "Today celebrates the nutritious leafy green that has been a staple in diets around the world. It is a reminder of the value of healthy eating."));
    m.insert("03-27", ("World Theatre Day", "This day celebrates the art of theatre and its role in reflecting and shaping society. It encourages appreciation for live performance and storytelling."));
    m.insert("03-28", ("National Something on a Stick Day", "Today is a lighthearted celebration of foods served on sticks, from lollipops to kebabs. It is a fun reminder that presentation can make eating more enjoyable."));
    m.insert("03-29", ("National Mom and Pop Business Owners Day", "This day honors small business owners who contribute to their communities. It encourages supporting local businesses and recognizing their impact."));
    m.insert("03-30", ("World Bipolar Day", "Today raises awareness about bipolar disorder and the importance of mental health support. It encourages understanding and reducing stigma around mental illness."));
    m.insert("03-31", ("World Backup Day", "This day reminds us to back up our digital data to prevent loss. It is a practical reminder of the importance of protecting our digital lives."));

    // April
    m.insert("04-01", ("April Fools' Day", "Today is a day for pranks, jokes, and lighthearted humor. It is a reminder that laughter and playfulness are important parts of life."));
    m.insert("04-02", ("World Autism Awareness Day", "Today raises awareness about autism spectrum disorder and promotes acceptance and inclusion. It encourages understanding and support for individuals with autism."));
    m.insert("04-03", ("World Party Day", "This day celebrates the joy of coming together and celebrating life. It is a reminder that shared experiences and festivities strengthen community bonds."));
    m.insert("04-04", ("International Day for Mine Awareness", "Today raises awareness about the dangers of landmines and the importance of demining efforts. It highlights the need for a world free from these hidden threats."));
    m.insert("04-05", ("National Deep Dish Pizza Day", "Today celebrates the hearty, Chicago-style deep dish pizza. It is a fun occasion to enjoy a beloved comfort food with friends and family."));
    m.insert("04-06", ("International Day of Sport for Development and Peace", "This day recognizes the power of sport to promote peace and development. It highlights how athletic activity can bridge divides and inspire communities."));
    m.insert("04-07", ("World Health Day", "Today marks the founding of the World Health Organization and raises awareness about global health issues. It encourages everyone to take steps toward better health."));
    m.insert("04-08", ("Draw a Picture of a Bird Day", "This lighthearted day encourages creativity by inviting people to draw birds. It is a reminder that art can be a simple and joyful activity for all ages."));
    m.insert("04-09", ("National Unicorn Day", "Today celebrates the magical and mythical unicorn. It is a fun reminder to embrace imagination and the wonder of fantastical thinking."));
    m.insert("04-10", ("National Siblings Day", "This day celebrates the special bond between siblings. It is an occasion to appreciate and honor the people who share your family history."));
    m.insert("04-11", ("National Pet Day", "Today celebrates the joy and companionship that pets bring into our lives. It is a reminder to cherish and care for our animal companions."));
    m.insert("04-12", ("International Day of Human Space Flight", "This day commemorates the first human journey into space. It celebrates human curiosity, courage, and the spirit of exploration."));
    m.insert("04-13", ("National Scrabble Day", "Today celebrates the classic word game that challenges vocabulary and strategy. It is a fun occasion to play and appreciate the richness of language."));
    m.insert("04-14", ("National Dolphin Day", "This day raises awareness about dolphins and the importance of protecting marine life. It celebrates the intelligence and playfulness of these remarkable animals."));
    m.insert("04-15", ("World Art Day", "Today celebrates the importance of creative arts in human culture. It encourages people to engage with art in all its forms and appreciate its power to inspire."));
    m.insert("04-16", ("National Librarian Day", "This day honors librarians who provide access to knowledge and foster a love of reading. It recognizes their vital role in communities and education."));
    m.insert("04-17", ("World Hemophilia Day", "Today raises awareness about hemophilia and other bleeding disorders. It highlights the importance of access to treatment and support for those affected."));
    m.insert("04-18", ("World Heritage Day", "This day celebrates the cultural and natural heritage of our world. It encourages the preservation of historic sites and traditions for future generations."));
    m.insert("04-19", ("National Garlic Day", "Today celebrates the flavorful and health-promoting properties of garlic. It is a fun occasion to appreciate this culinary staple used in cuisines around the world."));
    m.insert("04-20", ("National Look-Alike Day", "This lighthearted day celebrates the fun of resemblance and doubles. It is a reminder that even in our uniqueness, we can find surprising similarities with others."));
    m.insert("04-21", ("National Kindergarten Day", "Today honors the importance of early childhood education. It celebrates the foundational role that kindergarten plays in a child's development."));
    m.insert("04-22", ("Earth Day", "Today is a global celebration of our planet and a call to action for environmental protection. It encourages everyone to take steps to preserve the Earth for future generations."));
    m.insert("04-23", ("World Book Day", "This day celebrates books and reading as tools for education and imagination. It encourages people of all ages to discover the joy of reading."));
    m.insert("04-24", ("World Day for Laboratory Animals", "Today raises awareness about the use of animals in scientific research. It encourages the development of alternative methods and the humane treatment of laboratory animals."));
    m.insert("04-25", ("World Malaria Day", "This day raises awareness about the global impact of malaria and efforts to combat the disease. It highlights the importance of prevention and treatment in affected regions."));
    m.insert("04-26", ("World Intellectual Property Day", "Today celebrates the role of intellectual property in encouraging innovation and creativity. It highlights the importance of protecting ideas and inventions."));
    m.insert("04-27", ("Tell a Story Day", "This day encourages the sharing of stories as a way to connect and communicate. It is a reminder of the power of narrative to inspire and bring people together."));
    m.insert("04-28", ("World Day for Safety and Health at Work", "Today raises awareness about the importance of workplace safety. It encourages organizations to create environments where workers are protected from harm."));
    m.insert("04-29", ("International Dance Day", "This day celebrates dance as a universal form of expression and communication. It encourages people to appreciate and participate in the joy of movement."));
    m.insert("04-30", ("International Jazz Day", "Today celebrates jazz music and its role in promoting peace and dialogue. It highlights the cultural significance of this uniquely American art form that has influenced music worldwide."));

    // May
    m.insert("05-01", ("International Workers' Day", "Today honors the contributions of workers around the world. It is a day to recognize the importance of labor rights and the dignity of work."));
    m.insert("05-02", ("World Tuna Day", "This day raises awareness about the importance of sustainable tuna fishing. It highlights the role of tuna in global food security and the need to protect fish populations."));
    m.insert("05-03", ("World Press Freedom Day", "Today celebrates the fundamental principles of press freedom. It is a reminder of the importance of a free and independent media in a democratic society."));
    m.insert("05-04", ("Star Wars Day", "Today celebrates the beloved Star Wars franchise and its cultural impact. Fans around the world mark the occasion with the famous phrase 'May the Fourth be with you.'"));
    m.insert("05-05", ("World Hand Hygiene Day", "This day promotes the importance of hand hygiene in preventing the spread of disease. It encourages simple habits that can have a significant impact on public health."));
    m.insert("05-06", ("No Diet Day", "Today encourages body acceptance and challenges diet culture. It is a reminder to appreciate our bodies and focus on health rather than appearance."));
    m.insert("05-07", ("World Athletics Day", "This day celebrates athletics and encourages participation in sports. It highlights the benefits of physical activity for health and wellbeing."));
    m.insert("05-08", ("World Red Cross Day", "Today honors the humanitarian work of the Red Cross and Red Crescent. It celebrates the volunteers and staff who provide assistance in times of crisis."));
    m.insert("05-09", ("Europe Day", "This day celebrates peace and unity in Europe. It marks the anniversary of the Schuman Declaration, which laid the foundation for the European Union."));
    m.insert("05-10", ("World Lupus Day", "Today raises awareness about lupus, a chronic autoimmune disease. It highlights the importance of research, support, and understanding for those affected."));
    m.insert("05-11", ("Eat What You Want Day", "This lighthearted day encourages people to enjoy their favorite foods without guilt. It is a reminder that balance and enjoyment are both part of a healthy relationship with food."));
    m.insert("05-12", ("International Nurses Day", "Today honors nurses and their vital contributions to healthcare. It celebrates the compassion, skill, and dedication that nurses bring to their work."));
    m.insert("05-13", ("World Cocktail Day", "This day celebrates the art of mixology and the enjoyment of cocktails. It is a fun occasion to appreciate the creativity that goes into crafting a great drink."));
    m.insert("05-14", ("World Migratory Bird Day", "Today raises awareness about the importance of migratory birds and the need to protect their habitats. It celebrates the remarkable journeys these birds undertake each year."));
    m.insert("05-15", ("International Day of Families", "This day celebrates the importance of families in society. It recognizes the diverse forms that families take and the vital role they play in our lives."));
    m.insert("05-16", ("International Day of Light", "Today celebrates the role of light in science, culture, and art. It highlights the importance of light-based technologies in advancing society."));
    m.insert("05-17", ("World Telecommunication Day", "This day celebrates the role of information and communication technologies in connecting the world. It highlights the importance of digital access for all."));
    m.insert("05-18", ("International Museum Day", "Today celebrates museums and their role in preserving and sharing cultural heritage. It encourages people to visit museums and engage with history and art."));
    m.insert("05-19", ("World Plant a Vegetable Garden Day", "This day encourages people to grow their own vegetables. It highlights the benefits of gardening for health, sustainability, and connection to nature."));
    m.insert("05-20", ("World Bee Day", "Today raises awareness about the importance of bees and other pollinators. It highlights the vital role they play in maintaining biodiversity and food security."));
    m.insert("05-21", ("World Cultural Diversity Day", "This day celebrates cultural diversity and the importance of intercultural dialogue. It encourages understanding and respect for different cultures and traditions."));
    m.insert("05-22", ("International Day for Biological Diversity", "Today raises awareness about the importance of biodiversity. It highlights the need to protect the variety of life on Earth for future generations."));
    m.insert("05-23", ("World Turtle Day", "This day celebrates turtles and tortoises and raises awareness about their conservation. It encourages people to learn about and protect these ancient reptiles."));
    m.insert("05-24", ("Brother's Day", "Today celebrates the bond between brothers. It is an occasion to appreciate and honor the brothers in our lives."));
    m.insert("05-25", ("Towel Day", "This day is a tribute to author Douglas Adams and his beloved Hitchhiker's Guide to the Galaxy. Fans carry a towel to honor his memory and the humor of his work."));
    m.insert("05-26", ("World Dracula Day", "Today celebrates the iconic vampire character created by Bram Stoker. It is a fun occasion for fans of gothic literature and horror."));
    m.insert("05-27", ("World Multiple Sclerosis Day", "This day raises awareness about multiple sclerosis and the challenges faced by those living with the condition. It encourages support and research for better treatments."));
    m.insert("05-28", ("Amnesty International Day", "Today celebrates the founding of Amnesty International and its work to protect human rights. It is a reminder of the importance of standing up for justice and dignity."));
    m.insert("05-29", ("International Day of United Nations Peacekeepers", "This day honors the men and women who serve in UN peacekeeping missions. It recognizes their courage and dedication to maintaining peace and security."));
    m.insert("05-30", ("World MS Day", "Today raises awareness about multiple sclerosis and the global community of people affected by it. It encourages solidarity and support for those living with MS."));
    m.insert("05-31", ("World No Tobacco Day", "This day raises awareness about the harmful effects of tobacco use. It encourages people to quit smoking and supports efforts to reduce tobacco consumption worldwide."));

    // June
    m.insert("06-01", ("Global Day of Parents", "Today honors parents and their selfless commitment to children. It recognizes the vital role parents play in nurturing and guiding the next generation."));
    m.insert("06-02", ("World Milk Day", "This day celebrates the importance of milk as a global food. It highlights the nutritional benefits of dairy and the role of the dairy industry in feeding the world."));
    m.insert("06-03", ("World Bicycle Day", "Today celebrates the bicycle as a simple, affordable, and sustainable means of transport. It encourages cycling for health, environment, and mobility."));
    m.insert("06-04", ("International Day of Innocent Children Victims of Aggression", "This day acknowledges the pain suffered by children around the world who are victims of physical, mental, and emotional abuse. It calls for the protection of children's rights."));
    m.insert("06-05", ("World Environment Day", "Today is the principal United Nations day for encouraging awareness and action for the protection of our environment. It is a global platform for public outreach."));
    m.insert("06-06", ("World Pest Day", "This day raises awareness about pest management and its importance for public health and agriculture. It highlights the role of responsible pest control."));
    m.insert("06-07", ("World Food Safety Day", "Today raises awareness about foodborne risks and promotes actions to ensure the food we eat is safe. It highlights the importance of food safety in everyday life."));
    m.insert("06-08", ("World Oceans Day", "This day celebrates the world's oceans and raises awareness about their importance. It encourages action to protect marine ecosystems and promote sustainable ocean use."));
    m.insert("06-09", ("World Accreditation Day", "Today highlights the role of accreditation in supporting government policy and facilitating global trade. It promotes the value of accreditation in ensuring quality and safety."));
    m.insert("06-10", ("World Gin Day", "This day celebrates gin and its rich history and cultural significance. It is a fun occasion for enthusiasts to appreciate the craft of gin making."));
    m.insert("06-11", ("National Making Life Beautiful Day", "Today encourages people to add beauty to the world around them. It is a reminder that small acts of creativity and kindness can make a big difference."));
    m.insert("06-12", ("World Day Against Child Labour", "This day raises awareness about child labor and the need to eliminate it. It calls for action to protect children's rights and ensure they have access to education."));
    m.insert("06-13", ("International Albinism Awareness Day", "Today raises awareness about albinism and the challenges faced by people with this condition. It promotes respect and inclusion for people with albinism."));
    m.insert("06-14", ("World Blood Donor Day", "This day celebrates blood donors and raises awareness about the need for safe blood. It encourages people to donate blood and save lives."));
    m.insert("06-15", ("World Elder Abuse Awareness Day", "Today raises awareness about the abuse and neglect of older people. It calls for action to protect the rights and dignity of elderly individuals."));
    m.insert("06-16", ("International Day of the African Child", "This day commemorates the 1976 Soweto uprising and raises awareness about the need for improved education for African children. It celebrates the resilience and potential of African youth."));
    m.insert("06-17", ("World Day to Combat Desertification and Drought", "Today raises awareness about desertification and drought and their impact on communities. It encourages sustainable land management to prevent further degradation."));
    m.insert("06-18", ("International Picnic Day", "This day celebrates the joy of outdoor dining and the simple pleasure of sharing a meal in nature. It encourages people to spend time outdoors and enjoy the beauty of the natural world."));
    m.insert("06-19", ("World Sickle Cell Day", "Today raises awareness about sickle cell disease and the challenges faced by those affected. It highlights the importance of research and support for people living with this condition."));
    m.insert("06-20", ("World Refugee Day", "This day honors the courage, strength, and determination of refugees. It raises awareness about the challenges they face and calls for support and solidarity."));
    m.insert("06-21", ("World Music Day", "Today celebrates music and its power to bring people together. It encourages musicians of all levels to perform and share their music with the world."));
    m.insert("06-22", ("World Rainforest Day", "This day raises awareness about the importance of rainforests and the need to protect them. It highlights the vital role rainforests play in maintaining biodiversity and regulating the climate."));
    m.insert("06-23", ("International Olympic Day", "Today celebrates the Olympic movement and the values of excellence, respect, and friendship. It encourages participation in sports and physical activity."));
    m.insert("06-24", ("International Fairy Day", "This day celebrates the magical world of fairies and the power of imagination. It encourages people to embrace creativity and the wonder of fantastical thinking."));
    m.insert("06-25", ("Global Beatles Day", "Today celebrates the music and legacy of The Beatles. It honors their contribution to music and culture and the joy their songs have brought to generations of fans."));
    m.insert("06-26", ("International Day Against Drug Abuse and Illicit Trafficking", "This day raises awareness about the dangers of drug abuse and the importance of prevention. It calls for action to combat drug trafficking and support those affected by addiction."));
    m.insert("06-27", ("Micro-, Small and Medium-sized Enterprises Day", "Today celebrates the role of small and medium-sized enterprises in the global economy. It recognizes their contribution to innovation, employment, and sustainable development."));
    m.insert("06-28", ("International Body Piercing Day", "This day celebrates body piercing as a form of self-expression and art. It is a reminder that personal style and identity can be expressed in many ways."));
    m.insert("06-29", ("International Tropical Day", "Today celebrates the unique beauty and biodiversity of tropical regions. It raises awareness about the importance of protecting these ecosystems."));
    m.insert("06-30", ("World Social Media Day", "This day celebrates the impact of social media on global communication. It highlights how social platforms have transformed the way we connect and share information."));

    // July
    m.insert("07-01", ("International Joke Day", "Today celebrates the power of humor to bring joy and laughter. It encourages people to share jokes and appreciate the lighter side of life."));
    m.insert("07-02", ("World UFO Day", "This day encourages people to look up at the sky and ponder the possibility of extraterrestrial life. It celebrates curiosity about the universe and the unknown."));
    m.insert("07-03", ("International Plastic Bag Free Day", "Today raises awareness about the environmental impact of plastic bags. It encourages people to use reusable alternatives and reduce plastic waste."));
    m.insert("07-04", ("Independence Day (USA)", "Today marks the anniversary of the United States Declaration of Independence. It is a day of celebration, reflection, and patriotism for Americans around the world."));
    m.insert("07-05", ("World Bikini Day", "This day celebrates the bikini and its cultural impact since its introduction in 1946. It is a fun occasion to appreciate fashion and beach culture."));
    m.insert("07-06", ("World Zoonoses Day", "Today raises awareness about zoonotic diseases, which are transmitted between animals and humans. It highlights the importance of monitoring and preventing these diseases."));
    m.insert("07-07", ("World Chocolate Day", "This day celebrates the joy of chocolate and its rich history. It is a delicious occasion to appreciate one of the world's most beloved treats."));
    m.insert("07-08", ("Video Games Day", "Today celebrates the world of video games and their cultural impact. It is a fun occasion for gamers to enjoy their favorite games and appreciate the creativity of game developers."));
    m.insert("07-09", ("National Sugar Cookie Day", "This day celebrates the simple pleasure of sugar cookies. It is a sweet reminder that sometimes the most classic treats are the most satisfying."));
    m.insert("07-10", ("Nikola Tesla Day", "Today honors the life and legacy of inventor Nikola Tesla. It celebrates his contributions to science and technology, particularly in the field of electrical engineering."));
    m.insert("07-11", ("World Population Day", "This day raises awareness about global population issues and their impact on development. It encourages action to address challenges related to population growth and sustainability."));
    m.insert("07-12", ("Malala Day", "Today honors Malala Yousafzai and celebrates the power of education. It is a reminder of the importance of standing up for the right to education for all children."));
    m.insert("07-13", ("Embrace Your Geekness Day", "This day celebrates all things geeky and encourages people to embrace their passions. It is a reminder that enthusiasm and knowledge are things to be proud of."));
    m.insert("07-14", ("Bastille Day", "Today marks the anniversary of the storming of the Bastille in 1789, a pivotal moment in the French Revolution. It is celebrated as France's national day with festivities and fireworks."));
    m.insert("07-15", ("World Youth Skills Day", "This day raises awareness about the importance of equipping young people with skills for employment. It highlights the role of education and training in empowering youth."));
    m.insert("07-16", ("World Snake Day", "Today raises awareness about snakes and their importance in ecosystems. It encourages people to learn about and appreciate these often misunderstood reptiles."));
    m.insert("07-17", ("World Emoji Day", "This day celebrates the colorful world of emojis and their role in digital communication. It is a fun reminder of how these small symbols have transformed the way we express ourselves online."));
    m.insert("07-18", ("Nelson Mandela International Day", "Today honors the life and legacy of Nelson Mandela. It encourages people to dedicate 67 minutes to community service in honor of his 67 years of public service."));
    m.insert("07-19", ("National Hot Dog Day", "This day celebrates the beloved hot dog, a classic American food. It is a fun occasion to enjoy this simple pleasure and appreciate its place in food culture."));
    m.insert("07-20", ("International Chess Day", "Today celebrates the ancient game of chess and its role in developing strategic thinking. It encourages people to play and appreciate the intellectual challenge of chess."));
    m.insert("07-21", ("National Ice Cream Day", "This day celebrates the joy of ice cream in all its flavors and forms. It is a sweet occasion to indulge in a favorite treat and share the pleasure with others."));
    m.insert("07-22", ("World Brain Day", "Today raises awareness about brain health and neurological disorders. It encourages people to take care of their mental health and support research into brain diseases."));
    m.insert("07-23", ("World Whale Day", "This day raises awareness about whales and the importance of protecting marine life. It celebrates the majesty of these incredible creatures and the need to preserve their habitats."));
    m.insert("07-24", ("International Self-Care Day", "Today encourages people to prioritize their own health and wellbeing. It is a reminder that taking care of yourself is an important foundation for a healthy and fulfilling life."));
    m.insert("07-25", ("World Drowning Prevention Day", "This day raises awareness about drowning as a preventable cause of death. It encourages people to learn water safety skills and take steps to prevent drowning."));
    m.insert("07-26", ("Aunt and Uncle Day", "Today celebrates the special bond between aunts, uncles, and their nieces and nephews. It is an occasion to appreciate the important role extended family members play in our lives."));
    m.insert("07-27", ("National Take Your Pants for a Walk Day", "This lighthearted day encourages people to get outside and go for a walk. It is a fun reminder of the health benefits of regular physical activity."));
    m.insert("07-28", ("World Hepatitis Day", "Today raises awareness about viral hepatitis and its impact on global health. It encourages testing, prevention, and treatment to combat this disease."));
    m.insert("07-29", ("International Tiger Day", "This day raises awareness about the conservation of tigers and their habitats. It highlights the importance of protecting these magnificent animals from extinction."));
    m.insert("07-30", ("International Day of Friendship", "Today celebrates the importance of friendship in our lives. It encourages people to reach out to friends and strengthen the bonds that connect us."));
    m.insert("07-31", ("World Ranger Day", "This day honors park rangers and their dedication to protecting wildlife and natural habitats. It celebrates their vital role in conservation efforts around the world."));

    // August
    m.insert("08-01", ("World Lung Cancer Day", "Today raises awareness about lung cancer and the importance of early detection. It encourages people to learn about risk factors and support research for better treatments."));
    m.insert("08-02", ("International Beer Day", "This day celebrates beer and the brewers who craft it. It is a fun occasion to appreciate the diversity of beer styles and the culture that surrounds them."));
    m.insert("08-03", ("National Watermelon Day", "Today celebrates the refreshing joy of watermelon. It is a sweet reminder of summer pleasures and the simple enjoyment of fresh fruit."));
    m.insert("08-04", ("International Day of the World's Indigenous Peoples", "This day raises awareness about the rights and cultures of indigenous peoples. It celebrates their contributions to the world and calls for the protection of their rights."));
    m.insert("08-05", ("International Traffic Light Day", "Today marks the anniversary of the first electric traffic light. It is a reminder of how simple inventions can have a profound impact on daily life."));
    m.insert("08-06", ("Hiroshima Day", "Today commemorates the atomic bombing of Hiroshima in 1945. It is a day of remembrance and a call for nuclear disarmament and peace."));
    m.insert("08-07", ("National Lighthouse Day", "This day celebrates lighthouses and their role in maritime navigation. It honors the history and beauty of these iconic structures."));
    m.insert("08-08", ("International Cat Day", "Today celebrates cats and the joy they bring to our lives. It is a reminder to appreciate and care for our feline companions."));
    m.insert("08-09", ("International Day of the World's Indigenous Peoples", "This day celebrates the cultures and rights of indigenous peoples around the world. It calls for action to protect their lands, languages, and traditions."));
    m.insert("08-10", ("World Lion Day", "Today raises awareness about the conservation of lions. It highlights the importance of protecting these majestic animals and their habitats."));
    m.insert("08-11", ("World Calligraphy Day", "This day celebrates the art of beautiful writing. It encourages people to appreciate and practice calligraphy as a form of artistic expression."));
    m.insert("08-12", ("International Youth Day", "Today celebrates the potential of young people as partners in change. It raises awareness about the challenges facing youth and highlights their contributions to society."));
    m.insert("08-13", ("International Left-Handers Day", "This day celebrates left-handed people and raises awareness about the challenges they face in a right-handed world. It is a reminder to appreciate the diversity of human experience."));
    m.insert("08-14", ("World Lizard Day", "Today celebrates lizards and their fascinating diversity. It encourages people to learn about and appreciate these remarkable reptiles."));
    m.insert("08-15", ("National Relaxation Day", "This day encourages people to slow down and take time to relax. It is a reminder that rest and leisure are important for health and wellbeing."));
    m.insert("08-16", ("National Tell a Joke Day", "Today celebrates the power of humor to bring joy and laughter. It encourages people to share jokes and appreciate the lighter side of life."));
    m.insert("08-17", ("National Nonprofit Day", "This day celebrates the work of nonprofit organizations and the volunteers who support them. It recognizes their vital role in addressing social challenges and improving communities."));
    m.insert("08-18", ("World Honey Bee Day", "Today raises awareness about the importance of honey bees and their role in pollination. It highlights the need to protect these vital insects and their habitats."));
    m.insert("08-19", ("World Humanitarian Day", "This day honors humanitarian workers who risk their lives to help others. It raises awareness about the importance of humanitarian aid and the challenges faced by those in need."));
    m.insert("08-20", ("World Mosquito Day", "Today commemorates the discovery that mosquitoes transmit malaria. It raises awareness about mosquito-borne diseases and the importance of prevention and control."));
    m.insert("08-21", ("World Senior Citizen Day", "This day celebrates older adults and their contributions to society. It encourages respect and appreciation for the wisdom and experience of senior citizens."));
    m.insert("08-22", ("World Plant Milk Day", "Today celebrates plant-based milk alternatives and their benefits for health and the environment. It encourages people to explore dairy-free options."));
    m.insert("08-23", ("International Day for the Remembrance of the Slave Trade and its Abolition", "This day commemorates the slave trade and its abolition. It is a reminder of the importance of human dignity and the ongoing struggle for equality."));
    m.insert("08-24", ("International Strange Music Day", "This day celebrates unusual and experimental music. It encourages people to explore new sounds and appreciate the diversity of musical expression."));
    m.insert("08-25", ("World Water Week", "This week raises awareness about global water issues and the importance of sustainable water management. It encourages action to ensure clean water access for all."));
    m.insert("08-26", ("Women's Equality Day", "Today marks the anniversary of the 19th Amendment, which granted women the right to vote in the United States. It celebrates the progress made toward gender equality and the work still to be done."));
    m.insert("08-27", ("National Just Because Day", "This lighthearted day encourages people to do something nice for no particular reason. It is a reminder that acts of kindness don't need a special occasion."));
    m.insert("08-28", ("World Dream Day", "Today encourages people to pursue their dreams and inspire others to do the same. It is a reminder that dreams can drive positive change in the world."));
    m.insert("08-29", ("International Day Against Nuclear Tests", "This day raises awareness about the dangers of nuclear testing and calls for a world free from nuclear weapons. It commemorates the closure of the Semipalatinsk nuclear test site."));
    m.insert("08-30", ("International Day of the Disappeared", "Today raises awareness about enforced disappearances and the suffering of victims and their families. It calls for accountability and justice for those affected."));
    m.insert("08-31", ("International Overdose Awareness Day", "This day raises awareness about overdose and reduces the stigma of drug-related deaths. It acknowledges the grief felt by families and friends of those who have died or suffered from overdose."));

    // September
    m.insert("09-01", ("World Letter Writing Day", "Today celebrates the art of letter writing and the personal connection it creates. It encourages people to put pen to paper and reach out to someone special."));
    m.insert("09-02", ("World Coconut Day", "This day celebrates the coconut and its many uses in food, culture, and industry. It highlights the importance of coconut cultivation for communities around the world."));
    m.insert("09-03", ("World Beard Day", "Today celebrates beards and the men who wear them. It is a fun occasion to appreciate the diversity of facial hair styles and the culture that surrounds them."));
    m.insert("09-04", ("National Wildlife Day", "This day raises awareness about the importance of wildlife conservation. It encourages people to learn about and support efforts to protect endangered species."));
    m.insert("09-05", ("International Day of Charity", "Today celebrates the power of charity to transform lives. It encourages people to donate to causes they care about and support those in need."));
    m.insert("09-06", ("Read a Book Day", "This day celebrates the joy of reading and the power of books to educate and inspire. It encourages people to pick up a book and lose themselves in a good story."));
    m.insert("09-07", ("World Beard Day", "Today celebrates beards and the culture that surrounds them. It is a fun occasion to appreciate the diversity of facial hair and the men who wear it."));
    m.insert("09-08", ("International Literacy Day", "This day raises awareness about the importance of literacy for individuals and society. It highlights the need to ensure that everyone has access to quality education."));
    m.insert("09-09", ("World First Aid Day", "Today raises awareness about the importance of first aid and the role it plays in saving lives. It encourages people to learn basic first aid skills."));
    m.insert("09-10", ("World Suicide Prevention Day", "This day raises awareness about suicide prevention and the importance of mental health support. It encourages open conversations about mental health and the resources available to those in need."));
    m.insert("09-11", ("Patriot Day (USA)", "Today commemorates the September 11, 2001 attacks and honors the victims, survivors, and heroes of that day. It is a day of remembrance and reflection."));
    m.insert("09-12", ("National Day of Encouragement", "This day celebrates the power of encouragement to uplift and inspire others. It encourages people to offer words of support and positivity to those around them."));
    m.insert("09-13", ("International Chocolate Day", "Today celebrates the joy of chocolate and its rich history. It is a delicious occasion to appreciate one of the world's most beloved treats."));
    m.insert("09-14", ("World First Aid Day", "This day raises awareness about the importance of first aid and encourages people to learn life-saving skills. It highlights the role of first aid in emergency situations."));
    m.insert("09-15", ("International Day of Democracy", "Today celebrates the principles of democracy and the importance of civic participation. It encourages people to engage with democratic processes and uphold democratic values."));
    m.insert("09-16", ("World Ozone Day", "This day raises awareness about the depletion of the ozone layer and the importance of protecting it. It celebrates the progress made through the Montreal Protocol."));
    m.insert("09-17", ("International Country Music Day", "Today celebrates country music and its cultural heritage. It is a fun occasion to appreciate the storytelling and musical traditions of this beloved genre."));
    m.insert("09-18", ("World Water Monitoring Day", "This day encourages people to test and monitor local water bodies. It raises awareness about water quality and the importance of protecting water resources."));
    m.insert("09-19", ("International Talk Like a Pirate Day", "This lighthearted day encourages people to speak like pirates. It is a fun reminder not to take life too seriously and to embrace a bit of playful silliness."));
    m.insert("09-20", ("World Cleanup Day", "Today encourages people around the world to clean up their communities. It is a reminder that small actions can have a big impact on the environment."));
    m.insert("09-21", ("International Day of Peace", "This day celebrates peace and calls for a cessation of hostilities worldwide. It encourages people to work together to build a more peaceful world."));
    m.insert("09-22", ("World Car-Free Day", "Today encourages people to leave their cars at home and use alternative forms of transportation. It raises awareness about the environmental impact of car use."));
    m.insert("09-23", ("International Day of Sign Languages", "This day celebrates sign languages and the rights of deaf people. It raises awareness about the importance of sign language in the lives of deaf communities."));
    m.insert("09-24", ("World Gorilla Day", "Today raises awareness about the conservation of gorillas. It highlights the importance of protecting these endangered primates and their habitats."));
    m.insert("09-25", ("World Pharmacists Day", "This day celebrates pharmacists and their vital role in healthcare. It recognizes their expertise and dedication to patient wellbeing."));
    m.insert("09-26", ("World Contraception Day", "Today raises awareness about contraception and reproductive health. It encourages informed decision-making and access to family planning services."));
    m.insert("09-27", ("World Tourism Day", "This day celebrates tourism and its role in promoting cultural exchange and economic development. It encourages responsible and sustainable travel."));
    m.insert("09-28", ("World Rabies Day", "Today raises awareness about rabies and the importance of prevention. It highlights the need for vaccination and education to eliminate this deadly disease."));
    m.insert("09-29", ("World Heart Day", "This day raises awareness about cardiovascular disease and the importance of heart health. It encourages people to make lifestyle changes to protect their hearts."));
    m.insert("09-30", ("International Translation Day", "Today celebrates translators and their vital role in facilitating communication across languages. It recognizes the importance of translation in connecting people and cultures."));

    // October
    m.insert("10-01", ("International Day of Older Persons", "Today celebrates older people and their contributions to society. It raises awareness about the challenges they face and calls for respect and inclusion."));
    m.insert("10-02", ("International Day of Non-Violence", "This day celebrates non-violence and the legacy of Mahatma Gandhi. It encourages people to resolve conflicts peacefully and promote a culture of non-violence."));
    m.insert("10-03", ("World Habitat Day", "Today raises awareness about the state of human settlements and the right to adequate shelter. It encourages action to create sustainable and inclusive communities."));
    m.insert("10-04", ("World Animal Day", "This day raises awareness about animal welfare and the importance of protecting all species. It encourages people to take action to improve the lives of animals."));
    m.insert("10-05", ("World Teachers' Day", "Today celebrates teachers and their vital role in education. It recognizes their dedication and the profound impact they have on the lives of their students."));
    m.insert("10-06", ("World Cerebral Palsy Day", "This day raises awareness about cerebral palsy and the challenges faced by those affected. It highlights the importance of inclusion and support for people with cerebral palsy."));
    m.insert("10-07", ("World Smile Day", "Today celebrates the power of a smile to brighten someone's day. It encourages people to perform acts of kindness and spread joy."));
    m.insert("10-08", ("World Octopus Day", "This day celebrates octopuses and their remarkable intelligence. It encourages people to learn about and appreciate these fascinating marine creatures."));
    m.insert("10-09", ("World Post Day", "Today celebrates the postal service and its role in connecting people around the world. It highlights the importance of communication and the history of postal systems."));
    m.insert("10-10", ("World Mental Health Day", "This day raises awareness about mental health and the importance of mental wellbeing. It encourages people to seek help and support those struggling with mental health challenges."));
    m.insert("10-11", ("International Day of the Girl Child", "Today celebrates girls and their rights. It raises awareness about the challenges girls face and calls for action to empower them."));
    m.insert("10-12", ("World Arthritis Day", "This day raises awareness about arthritis and its impact on daily life. It encourages people to learn about the condition and support those affected."));
    m.insert("10-13", ("International Day for Disaster Risk Reduction", "Today raises awareness about disaster risk reduction and the importance of building resilient communities. It encourages action to reduce the impact of natural disasters."));
    m.insert("10-14", ("World Standards Day", "This day celebrates the importance of international standards in facilitating trade and ensuring safety. It highlights the role of standardization in our daily lives."));
    m.insert("10-15", ("Global Handwashing Day", "Today raises awareness about the importance of handwashing for preventing disease. It encourages people to adopt good hand hygiene practices."));
    m.insert("10-16", ("World Food Day", "This day raises awareness about global food security and the importance of sustainable agriculture. It calls for action to end hunger and promote healthy diets."));
    m.insert("10-17", ("International Day for the Eradication of Poverty", "Today raises awareness about poverty and the importance of addressing its root causes. It calls for action to create a more equitable world."));
    m.insert("10-18", ("World Menopause Day", "This day raises awareness about menopause and its impact on women's health. It encourages open conversations and support for women going through this transition."));
    m.insert("10-19", ("World Pediatric Bone and Joint Day", "Today raises awareness about musculoskeletal conditions in children. It highlights the importance of early diagnosis and treatment for these conditions."));
    m.insert("10-20", ("World Osteoporosis Day", "This day raises awareness about osteoporosis and the importance of bone health. It encourages people to take steps to prevent this condition."));
    m.insert("10-21", ("World Iodine Deficiency Day", "Today raises awareness about iodine deficiency and its impact on health. It highlights the importance of adequate iodine intake for thyroid function."));
    m.insert("10-22", ("International Caps Lock Day", "This lighthearted day celebrates the caps lock key and its role in digital communication. It is a fun reminder of the quirks of keyboard culture."));
    m.insert("10-23", ("International Snow Leopard Day", "Today raises awareness about the conservation of snow leopards. It highlights the importance of protecting these elusive and endangered big cats."));
    m.insert("10-24", ("United Nations Day", "This day celebrates the founding of the United Nations and its role in promoting peace and cooperation. It highlights the importance of international collaboration."));
    m.insert("10-25", ("World Pasta Day", "Today celebrates pasta and its place in global cuisine. It is a delicious occasion to appreciate the diversity of pasta dishes and the cultures that created them."));
    m.insert("10-26", ("National Pumpkin Day", "This day celebrates the pumpkin and its many uses in food and decoration. It is a festive reminder of autumn and the harvest season."));
    m.insert("10-27", ("World Day for Audiovisual Heritage", "Today raises awareness about the importance of preserving audiovisual heritage. It highlights the role of film, television, and radio in documenting human history."));
    m.insert("10-28", ("International Animation Day", "This day celebrates the art of animation and its impact on culture and entertainment. It encourages people to appreciate the creativity and skill involved in animated storytelling."));
    m.insert("10-29", ("World Stroke Day", "Today raises awareness about stroke and the importance of prevention and early treatment. It encourages people to learn the signs of stroke and take action to reduce their risk."));
    m.insert("10-30", ("World Thrift Day", "This day celebrates the value of saving and responsible financial management. It encourages people to adopt thrifty habits and appreciate the benefits of frugality."));
    m.insert("10-31", ("Halloween", "Today is a festive celebration of costumes, candy, and spooky fun. It is a reminder that imagination and playfulness can transform even the most ordinary day into something magical."));

    // November
    m.insert("11-01", ("World Vegan Day", "Today celebrates veganism and its benefits for health and the environment. It encourages people to explore plant-based diets and consider the impact of their food choices."));
    m.insert("11-02", ("Day of the Dead", "This day honors deceased loved ones with colorful celebrations and offerings. It is a reminder that the memories of those we have lost continue to live on in our hearts."));
    m.insert("11-03", ("World Jellyfish Day", "Today celebrates jellyfish and their fascinating biology. It encourages people to learn about and appreciate these unique marine creatures."));
    m.insert("11-04", ("National Candy Day", "This day celebrates the joy of candy in all its sweet forms. It is a fun occasion to indulge in a favorite treat and share the pleasure with others."));
    m.insert("11-05", ("World Tsunami Awareness Day", "Today raises awareness about tsunamis and the importance of disaster preparedness. It encourages communities to develop plans to protect themselves from these powerful natural events."));
    m.insert("11-06", ("International Day for Preventing the Exploitation of the Environment in War and Armed Conflict", "This day raises awareness about the environmental impact of war. It calls for action to protect natural resources during and after conflicts."));
    m.insert("11-07", ("International Merlot Day", "Today celebrates Merlot wine and the art of winemaking. It is a fun occasion for wine enthusiasts to appreciate this popular grape variety."));
    m.insert("11-08", ("World Urbanism Day", "This day celebrates urban planning and the importance of creating livable cities. It encourages people to think about how cities can be designed to improve quality of life."));
    m.insert("11-09", ("World Freedom Day", "Today commemorates the fall of the Berlin Wall and celebrates freedom. It is a reminder of the power of people to overcome oppression and build a better world."));
    m.insert("11-10", ("World Science Day for Peace and Development", "This day highlights the importance of science in promoting peace and sustainable development. It encourages the use of scientific knowledge to address global challenges."));
    m.insert("11-11", ("Remembrance Day", "Today honors the men and women who have served and sacrificed in military service. It is a day of reflection and gratitude for their courage and dedication."));
    m.insert("11-12", ("World Pneumonia Day", "This day raises awareness about pneumonia and its impact on global health. It highlights the importance of prevention, diagnosis, and treatment."));
    m.insert("11-13", ("World Kindness Day", "Today celebrates kindness and its power to transform lives. It encourages people to perform acts of kindness and spread positivity."));
    m.insert("11-14", ("World Diabetes Day", "This day raises awareness about diabetes and the importance of prevention and management. It highlights the global impact of this condition and calls for action."));
    m.insert("11-15", ("World Recycling Day", "Today raises awareness about the importance of recycling in reducing waste and conserving resources. It encourages people to adopt sustainable habits."));
    m.insert("11-16", ("International Day for Tolerance", "This day celebrates tolerance and the importance of respecting diversity. It encourages people to embrace differences and build bridges of understanding."));
    m.insert("11-17", ("World Prematurity Day", "Today raises awareness about premature birth and the challenges faced by premature babies and their families. It highlights the importance of research and support."));
    m.insert("11-18", ("World Antimicrobial Awareness Week", "This week raises awareness about antimicrobial resistance and the importance of responsible antibiotic use. It calls for action to preserve the effectiveness of these vital medicines."));
    m.insert("11-19", ("World Toilet Day", "Today raises awareness about the global sanitation crisis and the importance of access to clean toilets. It highlights the role of sanitation in public health."));
    m.insert("11-20", ("Universal Children's Day", "This day celebrates children and their rights. It raises awareness about the challenges children face and calls for action to protect and empower them."));
    m.insert("11-21", ("World Television Day", "Today celebrates television and its role in shaping culture and communication. It highlights the power of visual media to inform and inspire."));
    m.insert("11-22", ("International Stop Violence Against Women Day", "This day raises awareness about violence against women and calls for action to end it. It encourages people to stand up against gender-based violence."));
    m.insert("11-23", ("World Fisheries Day", "Today celebrates the importance of fisheries for food security and livelihoods. It highlights the need for sustainable fishing practices to protect marine ecosystems."));
    m.insert("11-24", ("Evolution Day", "This day celebrates the publication of Charles Darwin's 'On the Origin of Species' and the theory of evolution. It encourages appreciation for scientific inquiry and discovery."));
    m.insert("11-25", ("International Day for the Elimination of Violence Against Women", "Today raises awareness about violence against women and calls for action to end it. It is the start of 16 Days of Activism against Gender-Based Violence."));
    m.insert("11-26", ("World Olive Tree Day", "This day celebrates the olive tree and its cultural and environmental significance. It highlights the importance of olive trees in Mediterranean cultures and ecosystems."));
    m.insert("11-27", ("International Day of Epidemic Preparedness", "Today raises awareness about the importance of preparing for epidemics. It encourages governments and communities to develop plans to respond to disease outbreaks."));
    m.insert("11-28", ("Red Planet Day", "This day celebrates Mars and humanity's fascination with the Red Planet. It encourages people to learn about space exploration and the possibility of life beyond Earth."));
    m.insert("11-29", ("International Day of Solidarity with the Palestinian People", "Today raises awareness about the situation of the Palestinian people. It calls for a peaceful resolution to the conflict and respect for human rights."));
    m.insert("11-30", ("Computer Security Day", "This day raises awareness about the importance of computer security. It encourages people to take steps to protect their digital information and stay safe online."));

    // December
    m.insert("12-01", ("World AIDS Day", "Today raises awareness about HIV/AIDS and the importance of prevention and treatment. It honors those who have lost their lives to the disease and calls for action to end the epidemic."));
    m.insert("12-02", ("International Day for the Abolition of Slavery", "This day commemorates the adoption of the UN Convention for the Suppression of the Traffic in Persons. It calls for action to end modern slavery and human trafficking."));
    m.insert("12-03", ("International Day of Persons with Disabilities", "Today raises awareness about the rights and wellbeing of people with disabilities. It encourages inclusion and the removal of barriers that prevent full participation in society."));
    m.insert("12-04", ("World Wildlife Conservation Day", "This day raises awareness about the importance of wildlife conservation. It encourages people to take action to protect endangered species and their habitats."));
    m.insert("12-05", ("World Soil Day", "Today raises awareness about the importance of soil for food security and ecosystem health. It highlights the need to protect and sustainably manage soil resources."));
    m.insert("12-06", ("National Miners' Day", "This day honors miners and their contributions to society. It recognizes the hard work and dedication of those who work in the mining industry."));
    m.insert("12-07", ("International Civil Aviation Day", "Today celebrates the importance of international civil aviation. It highlights the role of air travel in connecting people and facilitating global trade."));
    m.insert("12-08", ("Pretend to Be a Time Traveler Day", "This lighthearted day encourages people to imagine traveling through time. It is a fun reminder to embrace creativity and the wonder of science fiction."));
    m.insert("12-09", ("International Anti-Corruption Day", "Today raises awareness about corruption and the importance of transparency and accountability. It calls for action to combat corruption at all levels of society."));
    m.insert("12-10", ("Human Rights Day", "This day celebrates the adoption of the Universal Declaration of Human Rights. It is a reminder of the fundamental rights and freedoms that belong to every person."));
    m.insert("12-11", ("International Mountain Day", "Today celebrates mountains and their importance for biodiversity and water resources. It raises awareness about the challenges facing mountain communities and ecosystems."));
    m.insert("12-12", ("International Day of Neutrality", "This day celebrates neutrality as a means of building peace and promoting dialogue. It highlights the importance of neutral states in international relations."));
    m.insert("12-13", ("International Day of the Violin", "Today celebrates the violin and its role in music. It encourages people to appreciate the beauty and versatility of this beloved instrument."));
    m.insert("12-14", ("Monkey Day", "This day celebrates monkeys and all non-human primates. It raises awareness about primate conservation and the importance of protecting these intelligent animals."));
    m.insert("12-15", ("International Tea Day", "Today celebrates tea and its cultural significance around the world. It is a reminder of the simple pleasure of sharing a cup of tea with others."));
    m.insert("12-16", ("National Chocolate Covered Anything Day", "This day celebrates the joy of chocolate-covered treats. It is a sweet occasion to indulge in a favorite combination of flavors."));
    m.insert("12-17", ("Wright Brothers Day", "Today commemorates the first successful powered airplane flight by the Wright Brothers. It celebrates human ingenuity and the spirit of exploration."));
    m.insert("12-18", ("International Migrants Day", "This day raises awareness about the contributions of migrants and the challenges they face. It calls for respect and protection of migrants' rights."));
    m.insert("12-19", ("Look for an Evergreen Day", "Today encourages people to appreciate evergreen trees and their beauty. It is a reminder of the enduring presence of nature even in the coldest months."));
    m.insert("12-20", ("International Human Solidarity Day", "This day celebrates solidarity and the importance of working together to address global challenges. It highlights the power of collective action."));
    m.insert("12-21", ("Crossword Puzzle Day", "This day celebrates a classic activity that combines language and memory. It highlights the satisfaction of finding the right answer."));
    m.insert("12-22", ("Cookie Exchange Day", "This day celebrates sharing homemade treats with others. It reflects the joy of generous traditions centered around food."));
    m.insert("12-23", ("Roots Day", "This day encourages people to reflect on where they come from. It is a reminder that identity can be shaped by connections across generations."));
    m.insert("12-24", ("Last-Minute Shopper's Day", "This day recognizes the familiar rush of people making their final purchases just before celebrations begin. It reflects the excitement of preparing gifts in the final hours."));
    m.insert("12-25", ("A'Phabet Day or No 'L' Day", "This day is built around a pun on the word Noel, which sounds like 'No L.' It invites people to experiment with language by trying to avoid using the letter L."));
    m.insert("12-26", ("Boxing Day", "Today is traditionally associated with giving and generosity through the continuation of holiday celebrations. In many countries, it is also known as a relaxed day of family gatherings and public events."));
    m.insert("12-27", ("Visit the Zoo Day", "Today encourages us to spend time observing animals up close. It is a day that celebrates the role zoos can have in education through the preservation of species."));
    m.insert("12-28", ("Card Playing Day", "This day celebrates games that bring people together through strategy. It highlights the simple enjoyment of spending time with others around a table."));
    m.insert("12-29", ("Tick Tock Day", "Today is a reminder that the year is almost over and that there is still time to finish important unfinished tasks. It encourages a final push to wrap things up before the new year begins."));
    m.insert("12-30", ("National Resolution Planning Day", "Today encourages us to reflect on the year that is ending and thoughtfully prepare goals for the year ahead. It is a day focused on planning with clearer, more practical steps."));
    m.insert("12-31", ("New Year's Eve", "New Year's Eve marks the closing of the year and the anticipation of a new beginning. It is often a time for reflection by looking ahead with hope."));

    m
}

// ── Public API ───────────────────────────────────────────────────────────────

/// Returns the [GlobalEvent] for today's date.
/// Stable for the entire calendar day — no random selection.
pub fn get_today_event() -> GlobalEvent {
    get_event_for_date(Local::now().date_naive())
}

/// Returns the [GlobalEvent] for a specific date.
/// Falls back to a safe placeholder if the key is somehow missing.
pub fn get_event_for_date(date: NaiveDate) -> GlobalEvent {
    let key = format!("{:02}-{:02}", date.month(), date.day());
    let map = build_event_map();
    match map.get(key.as_str()) {
        Some((title, description)) => GlobalEvent {
            title: title.to_string(),
            description: description.to_string(),
        },
        None => GlobalEvent {
            title: "Global Event".to_string(),
            description: "Today is a day to celebrate and reflect.".to_string(),
        },
    }
}

/// Returns only the title of tomorrow's event.
/// Handles year wrap: December 31 → January 1.
pub fn get_tomorrow_title() -> String {
    let tomorrow = Local::now().date_naive() + chrono::Duration::days(1);
    get_event_for_date(tomorrow).title
}