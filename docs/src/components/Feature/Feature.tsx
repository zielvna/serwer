import Heading from '@theme/Heading';
import styles from './Feature.module.css';

type Props = {
  icon: React.ReactElement;
  title: string;
  description: string;
};

export default function Feature({ icon, title, description }: Props) {
  return (
    <div className={styles.feature}>
      {icon}
      <div className={styles.featureText}>
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}
