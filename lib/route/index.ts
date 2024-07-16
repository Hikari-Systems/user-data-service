import express from 'express';
import userRoutes from './user';
import oauthProfileRoutes from './oauth_profile';

const router = express.Router();

router.use('/user', userRoutes);
router.use('/oauthProfile', oauthProfileRoutes);

export default router;
