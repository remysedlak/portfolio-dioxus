const portfolio = [
    {
        imgUrl: 'assets/snapshots/inquisitor.jpg',
        title: 'The Inqusitor',
        stack: [
            { name: 'python', color: '#B3D3F2', link: 'https://www.python.org/' },
            { name: 'js', color: '#FFD699', link: 'https://developer.mozilla.org/en-US/docs/Web/JavaScript' },
            { name: 'html5', color: '#FFD699', link: 'https://developer.mozilla.org/en-US/docs/Web/HTML' },
        ],
       repository: 'https://github.com/DSmith215/The-Inquistor',
        desc: 'The Inquisitor is a demo extension that detect\'s <b>deepfake images</b> of politicians to stop <b>misinformation</b> online. User\'s can optimize filters on the module.'
    },
    {
        imgUrl: 'assets/snapshots/reflexa.jpg',
        title: 'Reflexa',
        stack: [
            { name: 'react', color: '#B3ECFF', link: 'https://react.dev/' },
            { name: 'django', color: '#B3C6B3', link: 'https://www.djangoproject.com/' },
            { name: 'postgresql', color: '#B3C6E0', link: 'https://www.postgresql.org/' },
            { name: 'ec2', color: '#FFE5B4', link: 'https://aws.amazon.com/ec2/' },
        ],
        repository: 'https://github.com/remysedlak/Reflexa',
        desc: 'Reflexa is a <b>full stack web application</b> designed to analyze <b>mental health</b> journal entries overtime. Data analysis is used to evaluate journal content and emotion.'
    },

    {
        imgUrl: 'assets/snapshots/tkinter.jpg',
        title: 'TkinterAudio',
        stack: [
            { name: 'python', color: '#B3D3F2', link: 'https://www.python.org/' },
        ],
        repository:'https://github.com/remysedlak/tkinter-audio-analysis',
        desc: "Python <b>GUI</b>, plays local <b>audio files</b> and generates plots, extracting audio features via <a target='_blank' href='https://librosa.org/doc/latest/index.html'>Librosa</a>'s Fourier transform. Features include amplitude, chroma, zcr, rms, etc."
    },

    {
        imgUrl: 'assets/snapshots/audio-hub.jpg',
        title: 'Audio Hub',
        stack: [
            { name: 'react', color: '#B3ECFF', link: 'https://react.dev/' },
            { name: 'vitejs', color: '#D6D6FF', link: 'https://vitejs.dev/' },
            { name: 'tailwindcss', color: '#B2F5EA', link: 'https://tailwindcss.com/' },
        ],
        repository:'https://github.com/remysedlak/audio-hub',
        desc: 'This js app is designed for <b>music producers</b> to evaluate infromation from <b>samples</b>. Currently developing a reliable browser-side spectrogram algorithm.'
    },
    {
        imgUrl: 'assets/snapshots/clipvault.png',
        title: 'ClipVault',
        stack: [
            { name: 'rust', color: '#B3ECFF', link: 'https://www.rust-lang.org/' },
            { name: 'sqlite', color: '#B3ECFF', link: 'https://www.sqlite.org/' },
        ],
       repository:'https://github.com/remysedlak/clipvault',
        desc: 'ClipVault is a <b>lightweight</b>, <b>privacy-focused</b> clipboard manager built in <b>Rust</b> with a native GUI using <b>egui</b> and <b>system tray integration</b>.'
    },
    {
        imgUrl: 'assets/snapshots/plant-mob.png',
        title: 'Plant Parent',
        stack: [
            { name: 'react', color: '#B3ECFF', link: 'https://react.dev/' },
            { name: 'vitejs', color: '#D6D6FF', link: 'https://vitejs.dev/' },
            { name: 'django', color: '#B3C6B3', link: 'https://www.djangoproject.com/' },
            { name: 'tailwindcss', color: '#B2F5EA', link: 'https://tailwindcss.com/' },
            { name: 'render', color: '#B2F5EA', link: 'https://render.com/' },
        ],
        repository:'https://github.com/remysedlak/plant-parent',
        desc: 'This webapp track <b>plant\'s</b> and their <b>needs</b>. Currently developing a login system and a database to store plant information and images.'
    },
    // Add more projects as needed
];
export default portfolio;