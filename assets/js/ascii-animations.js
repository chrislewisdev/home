const xMax = 38;
const yMax = 6;
const updateRateMs = 200;

let animations = [];

function getCharacter(element, x, y) {
    return element.children[y+1].textContent.split('')[x+1];
}

function setCharacter(element, x, y, character) {
    let line = element.children[y+1];

    let ascii = line.textContent.split('');
    ascii.splice(x+1, 1, character);
    line.textContent = ascii.join('');
}

function updateParticle(p, container, animator, character) {
    let [x, y] = p;
    if (getCharacter(container, x, y) === character) {
        setCharacter(container, x, y, '.');
    }

    [x, y] = animator(p);

    if (getCharacter(container, x, y) === '.') {
        setCharacter(container, x, y, character);
    }

    p[0] = x;
    p[1] = y;
}

function rainAnimator(p) {
    return [p[0], p[1] + 1];

}

function waveAnimator(p) {
    return [p[0] + 1, p[1]];
}

function shootingStarAnimator(p) {
    return [p[0] + 1, p[1] + 1];
}

function bubblesAnimator(p) {
    return [p[0], p[1] - 1];
}

function updateAnimations() {
    animations.forEach(a => {
        a.particles.forEach(p => {
            updateParticle(p, a.container, a.animator, a.character);
        });

        a.particles = a.particles.filter(s => 
            s[0] >= 0 && s[0] <= xMax && s[1] >= 0 && s[1] <= yMax
        );
    });
}

function topEdgeSpawner(target) {
    target.push([Math.floor(Math.random() * xMax), -1]);
}

function bottomEdgeSpawner(target) {
    target.push([Math.floor(Math.random() * xMax), yMax + 1]);
}

function leftEdgeSpawner(target) {
    target.push([-1, Math.floor(Math.random() * yMax)]);
}

window.addEventListener('DOMContentLoaded', () => {
    let logosOnPage = document.getElementsByClassName('animated');
    Array.prototype.forEach.call(logosOnPage, l => {
        let a = {
            container: l.children[0],
            particles: [],
            animator: rainAnimator,
            spawner: topEdgeSpawner,
            character: '*'
        };

        if (l.classList.contains('shooting-stars')) {
            a.animator = shootingStarAnimator;
            a.character = 'o';
        } else if (l.classList.contains('waves')) {
            a.animator = waveAnimator;
            a.spawner = leftEdgeSpawner;
            a.character = '~';
        } else if (l.classList.contains('bubbles')) {
            a.animator = bubblesAnimator;
            a.spawner = bottomEdgeSpawner;
            a.character = 'o';
        }

        setInterval(() => {
            a.spawner(a.particles);
        }, updateRateMs);

        animations.push(a);
    });

    setInterval(updateAnimations, updateRateMs);
});
